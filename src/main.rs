/// This is the entrypoint for a program meant for development and testing of features.
/// It may later become a CLI script for scholarwander, but feel free to use it and hack on it for development and testing

use clap::Parser;
use std::env;
use std::fs;
use std::fmt;
use openssl::cms;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to cloudconfig file
    #[arg(short, long)]
    cloudconfig: String,
}

fn main() {
    let args = Args::parse();

    
    let cloudconfig_cipherdata: Vec<u8> = fs::read(args.cloudconfig)
        .expect("Should have been able to read the file");

    let config = cms::CMSOptions::NO_SIGNER_CERT_VERIFY;
    // config::set()

    if let Ok(mut cmsinfo) = cms::CmsContentInfo::from_der(&cloudconfig_cipherdata) {

        let mut outData:Vec<u8> = vec![];
        if let Ok(result) = cmsinfo.verify(
            None, //certs: 
            None, //store: 
            None, //detached_data: 
            Some(&mut outData), //output_data: 
            config //flags: 
        ){

            if let Ok(res_str) = String::from_utf8(outData) {
                println!("{}", res_str);

            }

        }

    }
    
}