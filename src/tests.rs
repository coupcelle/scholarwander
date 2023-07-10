use crate::parse_xml;

#[test]
fn parsing() {
    parse_xml(
        "<paladinResponse xmlns=\"http://schemas.securew2.com/paladinResponse\" type=\"7\" version=\"1.2.1\"></paladinResponse>"
        .bytes()
        .collect::<Vec<_>>()
        .as_slice()
    ).unwrap();
}
