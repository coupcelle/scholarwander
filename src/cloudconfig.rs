
use xml::reader::{EventReader, XmlEvent};


#[derive(Debug)]
pub struct CloudConfig {
    
}

impl CloudConfig {

	pub fn from_xml<T: std::io::Read>(xml: T) -> Result<CloudConfig, xml::reader::Error> {
		let mut depth = 0;
		let xml = EventReader::new(xml);
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
		Ok(CloudConfig {})
	}
}