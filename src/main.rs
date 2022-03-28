use std::io::{BufRead, BufReader};
use std::net::Ipv4Addr;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

use adsb_deku::deku::DekuContainerRead;
use adsb_deku::{Frame, DF};
use clap::Parser;
use rsadsb_common::{AirplaneDetails, Airplanes};

#[derive(Debug, Clone, Parser)]
#[clap(
    version,
    name = "elons-jets-rs",
    author = "wcampbell0x2a",
    about = "Track Elon Musk's registered airplanes"
)]
struct Opts {
    /// ip address / hostname of ADS-B server / demodulator
    #[clap(long, default_value = "127.0.0.1")]
    host: Ipv4Addr,

    /// port of ADS-B server / demodulator
    #[clap(long, default_value = "30002")]
    port: u16,

    /// Antenna location latitude, this use for aircraft position algorithms.
    #[clap(long)]
    lat: f64,

    /// Antenna location longitude
    #[clap(long)]
    long: f64,
}

fn main() {
    let opts = Opts::parse();
    let host = opts.host;
    let port = opts.port;

    let socket = SocketAddr::from((host, port));
    let stream = TcpStream::connect_timeout(&socket, Duration::from_secs(5)).unwrap_or_else(|_|
        panic!(r#"could not open port to ADS-B client at {host}:{port}, try running https://github.com/rsadsb/dump1090_rs.
see https://github.com/rsadsb/adsb_deku#serverdemodulationexternal-applications for more details"#)
    );
    stream
        .set_read_timeout(Some(std::time::Duration::from_millis(50)))
        .unwrap();
    let mut reader = BufReader::new(stream);
    let mut input = String::new();
    println!("connected, waiting for elon musk...");

    let mut adsb_airplanes = Airplanes::new();
    loop {
        if let Ok(len) = reader.read_line(&mut input) {
            // a length of 0 would indicate a broken pipe/input, quit program
            if len == 0 {
                break;
            }

            // convert from string hex -> bytes
            let hex = &mut input.to_string()[1..len - 2].to_string();
            let bytes = if let Ok(bytes) = hex::decode(&hex) {
                bytes
            } else {
                continue;
            };

            // check for all 0's
            if bytes.iter().all(|&b| b == 0) {
                continue;
            }

            // parse the entire DF frame
            let frame = Frame::from_bytes((&bytes, 0));
            if let Ok((_, frame)) = frame {
                if let DF::ADSB(ref adsb) = frame.df {
                    // filter through elon musk's jets
                    if [
                        [0xa8_u8, 0x35, 0xaf],
                        [0xa2, 0xae, 0x0a],
                        [0xa6, 0x43, 0x04],
                    ]
                    .iter()
                    .any(|x| x == &adsb.icao.0)
                    {
                        adsb_airplanes.action(frame, (opts.lat, opts.long));
                    }
                }
            }
        }
        // for every found jet, print position
        for key in adsb_airplanes.keys() {
            let value = adsb_airplanes.aircraft_details(*key);
            if let Some(AirplaneDetails { position, .. }) = value {
                println!(
                    "SEEN: [{key}] - {} {}",
                    position.latitude, position.longitude
                );
            }
        }
        input.clear();
        // remove airplanes that timed-out
        adsb_airplanes.prune(30);
    }
}
