pub mod integer;
pub mod nil;

pub fn parse(s: &str) {
    println!("TODO!");
}

pub trait AtomParser<T> {
    fn str_is(&self, &str) -> bool;
    fn parse_str(&self, &str) -> T;
}

trait SExpr {}

struct Cons<A: SExpr, B: SExpr>(A, B);
pub trait AtomData {
    type T;
    fn get_value(&self) -> Self::T;
}

impl<A: SExpr, B: SExpr> SExpr for Cons<A, B> {}
impl<T: AtomData> SExpr for T {}
