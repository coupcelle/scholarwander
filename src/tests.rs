use std::io::BufReader;

use crate::parse_xml;

#[test]
fn parsing() {
    let bufreader = BufReader::new("<paladinResponse xmlns=\"http://schemas.securew2.com/paladinResponse\" type=\"7\" version=\"1.2.1\">
");
    parse_xml(bufreader)?;
}
