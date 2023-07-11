#![warn(clippy::nursery, clippy::pedantic)]
/// This is the entrypoint for a program meant for development and testing of features.
/// It may later become a CLI script for scholarwander, but feel free to use it and hack on it for development and testing
use clap::Parser;
use openssl::cms;
use std::env;
use std::fmt;
use std::fs;
use serde_xml_rs::{Error};
use serde_xml_rs::de::from_str;
use uuid::Uuid;

mod cloudconfig;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to cloudconfig file
    #[arg(short, long)]
    cloudconfig: String,
}

fn main() {
    let args = Args::parse();

    let config:CloudConfig = CloudConfig{};

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
            if let Ok(res_str) = String::from_utf8(outData.clone()) {
                // println!("{}", res_str);
                config = match cloudconfig::CloudConfig::from_xml(&res_str)  {
                    Ok(cfg) => cfg,
                    Err(error)=> {
                        let jd = &mut serde_xml_rs::de::Deserializer::new_from_reader(res_str.as_bytes());
                        let result: Result<cloudconfig::CloudConfig, _> = serde_path_to_error::deserialize(jd).map_err(|err| {
                            return println!("Failed to parse : {}", err);//'{}', res_str
                        });
                        panic!("{}", error)
                    }
                };
            
                println!("{}", config);
                if let Some(creds) = &config.configurations.deviceConfiguration[0].actions[0].action[1].credentials {
                    let enrollmentdata = match &creds.tlsEnrollment {
                        Some(tlsEnrollment) => &tlsEnrollment.webSSOUrl.as_str(),
                        None => "Web SSO is not used for auth"
                    };

                    //crude way to detect whether or not a webSSO url is present and enf the program early if it isnt. most of these urls are gonna be way ofer 30 chars
                    if enrollmentdata.len() < 30 {
                        panic!("{}", enrollmentdata);
                    }

                    println!("{}", enrollmentdata);
                    //get transaction ID (generate a new uuid4)
                    let id = Uuid::new_v4();
                    // open URL
                    let url = enrollmentdata.replace("%TRANSACTIONID%", &id.to_string());
                    println!("{}", url);

                }

            }
            
        }
    }
}
