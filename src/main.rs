#![warn(clippy::nursery, clippy::pedantic)]
/// This is the entrypoint for a program meant for development and testing of features.
/// It may later become a CLI script for scholarwander, but feel free to use it and hack on it for development and testing
use clap::Parser;
// use openssl::cms;
// use std::env;
// use std::fmt;
use std::{fs, io::BufReader};

// use std::io::{BufReader, Cursor};
use xml::reader::{EventReader, XmlEvent};

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

    // let config = cms::CMSOptions::NO_SIGNER_CERT_VERIFY;
    // config::set()

    // if let Ok(mut cmsinfo) = cms::CmsContentInfo::from_der(&cloudconfig_cipherdata) {
    //     let mut outData: Vec<u8> = Vec::new();
    //     if let Ok(result) = cmsinfo.verify(
    //         None,               //certs:
    //         None,               //store:
    //         None,               //detached_data:
    //         Some(&mut outData), //output_data:
    //         config,             //flags:
    //     ) {
    //         // if let Ok(res_str) = String::from_utf8(outData.clone()) {
    //         //     println!("{}", res_str);
    //         // }
    //         parse_xml(outData.as_slice())?
    //     }
    // }
}

fn parse_xml<T: std::io::Read>(xml: T) -> Result<usize, xml::reader::Error> {
    let mut depth = 0;
    // buffering is apparently important for performance
    let xml = EventReader::new(BufReader::new(xml));
    for e in xml {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{:spaces$}+{name}", "", spaces = depth * 2);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{:spaces$}-{name}", "", spaces = depth * 2);
            }
            Err(e) => {
                eprintln!("Error: {e}");
                return Err(e);
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }
    Ok(depth)
}
