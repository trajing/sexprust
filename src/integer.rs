use AtomData;
use AtomParser;

pub struct Integer {
    value: i32,
}

impl AtomData for Integer {
    type T = i32;

    fn get_value(&self) -> i32 {
        self.value
    }
}

pub struct IntegerParser;

impl AtomParser<Integer> for IntegerParser {
    fn str_is(&self, s: &str) -> bool {
        true
    }

    fn parse_str(&self, s: &str) -> Integer {
        Integer { value: 1234 }
    }
}
