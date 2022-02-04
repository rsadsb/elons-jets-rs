# Track Elon Musk's jets - in rust

With inpsiration from [twitter/@ElonJet](https://twitter.com/ElonJet), this app will print notifications
when one of Elon Musk's jets fly over your own ADS-B receiver ground station.

# Run
You will need an ADS-B reciever running, use [dump1090_rs](https://github.com/rsadsb/dump1090_rs)
for the best experience.

Once that is running, use the following command to start this application:
```
> cargo r -- --lat <lat> --long <long>
```

# Warning
This is untested, since I don't live near Elon Musk.

# Usage
```
elons-jets-rs 0.1.0
wcampbell0x2a
Track Elon Musk's registered airplanes

USAGE:
    elons-jets-rs [OPTIONS] --lat <LAT> --long <LONG>

OPTIONS:
    -h, --help           Print help information
        --host <HOST>    ip address / hostname of ADS-B server / demodulator [default: 127.0.0.1]
        --lat <LAT>      Antenna location latitude, this use for aircraft position algorithms
        --long <LONG>    Antenna location longitude
        --port <PORT>    port of ADS-B server / demodulator [default: 30002]
    -V, --version        Print version information
```
