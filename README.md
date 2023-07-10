# scholarwander

**Initial Release Target:**

Audience: LUG members and other techies interested in a FOSS alternativel to SecureW2
Platform: NetworkManager initially
Schools Supported: RIT and NCSU
Timeline: By Aug 10, 2023

## Building

This program links to OpenSSL. You should have openssl installed
the Debian/Ubuntu way to do this is using the command `sudo apt-get install pkg-config libssl-dev`, Commands for other systems can be found [here](https://docs.rs/openssl/latest/openssl/#automatic).

To build run `cargo build`


## Running

To run, use the command `cargo run -- --cloudconfig ../edurams/RIT/_RIT_SecureW2_JoinNow.run.extracted/tar_w2/SecureW2.cloudconfig` (assumes this repository and the `edurams` folder are located in the same folder)

