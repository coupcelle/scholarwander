
use crate::cloudconfig::CloudConfig;
use serde::{Deserialize};

use serde_xml_rs::{Error};
use serde_xml_rs::de::from_str;


pub fn handle_parse_error<'a, T: Deserialize<'a>>(parsed_string: String) {
	let jd = &mut serde_xml_rs::de::Deserializer::new_from_reader(parsed_string.as_bytes());
	let result: Result<T, _> = serde_path_to_error::deserialize(jd).map_err(|err| {
		return println!("Failed to parse : {}", err);//'{}', res_str
	});
	// panic!("{}", error)
}