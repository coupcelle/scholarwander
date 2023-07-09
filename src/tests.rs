use std::io::BufReader;

use xml::EventReader;

use crate::parse_xml;

#[test]
fn parsing() {
    parse_xml(
        EventReader::new(BufReader::new("<paladinResponse xmlns=\"http://schemas.securew2.com/paladinResponse\" type=\"7\" version=\"1.2.1\">
    ".bytes().collect::<Vec<_>>().as_slice()))
    ).unwrap();
}
