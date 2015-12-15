pub mod integer;

pub fn parse(s: &str) -> Expr {
    println!("TODO!")
}

pub trait AtomParser<T> {
    fn str_is(&self, &str) -> bool;
    fn parse_str(&self, &str) -> T;
}

pub trait AtomData<T> {
    fn get_value(&self) -> T;
}

pub enum Expr {
    Nil,
    SExpr(Expr),
    Atom(AtomData),
}
