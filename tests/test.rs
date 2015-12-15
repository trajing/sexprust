extern crate sexprust;
use sexprust::integer;
use sexprust::AtomData;
use sexprust::AtomParser;

#[test]
fn it_works() {
}

#[test]
fn default_parse() {
    assert_eq!(sexprust::parse("(+ (* 2 3) 4)", 10)
}

#[test]
fn integer_parse() {
    assert_eq!(integer::IntegerParser.str_is("1"), true)
    assert_eq!(integer::IntegerParser.parse_str("1"), 1)
}
