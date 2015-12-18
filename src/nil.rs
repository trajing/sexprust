use AtomData;
use AtomParser;

pub struct Nil;

impl AtomData for Nil {
    type T = bool;

    fn get_value(&self) -> bool {
        false
    }
}

pub struct NilParser;

impl AtomParser for NilParser {
    fn str_is(&self, s: &str) -> bool {
        s == "nil"
    }

    fn parse_str(&self, s: &str) -> Nil {
        Nil
    }
}
