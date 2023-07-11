# scholarwander

**Initial Release Target:**

Audience: LUG members and other techies interested in a FOSS alternativel to SecureW2

Platform: NetworkManager on a Debian/Ubuntu based OS

Schools Supported: RIT and NCSU

Timeline: By Aug 10, 2023

## Building

This program links to OpenSSL so you should have openssl installed. The Debian/Ubuntu way to do this is using the command `sudo apt-get install pkg-config libssl-dev`, Commands for other systems can be found [here](https://docs.rs/openssl/latest/openssl/#automatic).

This program also will (at some point in the future) link to networkManager. See [here](https://github.com/truehumandesign/networkmanager-rs#build-prerequisites) for full build prerequisites. Debian/Ubuntu users should ensure the following packages are installed `network-manager libdbus-1-dev pkg-config`.

This program also links to some [platform specific web libraries](https://github.com/tauri-apps/wry#platform-specific-notes) to support being able to display a browser for SSO. Debian/Ubuntu users should run the following command to ensure the packages are installed `sudo apt install libwebkit2gtk-4.1-dev`.

To build run `cargo build`


## Running/Usage

To run, use the command `cargo run -- --cloudconfig ../path/to/SecureW2.cloudconfig`


This configuration file `SecureW2.cloudconfig` is what the official SecureW2 client uses to know how to set up your connection. This is needed if you want to be able to run ScholarWander.

To get this config information, navigate to your universities portal for setting up wifi/eduroam (typically `wifi.<org>.edu` or `<org>.edu/wifi`). Select "Linux" as the download option and continue to the download. You should end up with a file ending in `.run`.
### Extracting the config from the `.run`
To extract the config file (`.cloudconfig`) from the installer, use the command `sed -e '0,/^#ARCHIVE#$/d' /path/to/file.run | tar zxf - SecureW2.cloudconfig`, this should spit out a `SecureW2.cloudconfig` file in your current directory.

