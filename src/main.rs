#![warn(clippy::nursery, clippy::pedantic)]
/// This is the entrypoint for a program meant for development and testing of features.
/// It may later become a CLI script for scholarwander, but feel free to use it and hack on it for development and testing
use clap::Parser;
use openssl::cms;
use std::env;
use std::fmt;
use std::fs;


mod cloudconfig;

#[cfg(test)]
mod tests;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to cloudconfig file
    #[arg(short, long)]
    cloudconfig: String,
}

fn main() {
    let args = Args::parse();

    let cloudconfig_cipherdata: Vec<u8> =
        fs::read(args.cloudconfig).expect("Should have been able to read the file");

    let config = cms::CMSOptions::NO_SIGNER_CERT_VERIFY;

    if let Ok(mut cmsinfo) = cms::CmsContentInfo::from_der(&cloudconfig_cipherdata) {
        let mut outData: Vec<u8> = Vec::new();
        if let Ok(result) = cmsinfo.verify(
            None,               //certs:
            None,               //store:
            None,               //detached_data:
            Some(&mut outData), //output_data:
            config,             //flags:
        ) {
            // if let Ok(res_str) = String::from_utf8(outData.clone()) {
            //     println!("{}", res_str);
            // }
            cloudconfig::CloudConfig::from_xml(outData.as_slice()).unwrap();
        }
    }
}
