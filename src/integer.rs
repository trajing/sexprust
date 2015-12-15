struct Integer {
    value: i32,
}

impl AtomData<i32> for Integer {
    fn get_value(&self) -> i32 {
        self.value
    }
}

struct IntegerParser;

impl AtomParser<Integer> for IntegerParser {
    fn str_is(&self, s: &str) {
        true
    }

    fn parse_str(&self, s: &str) {
        Integer { value: 1234 }
    }
}
