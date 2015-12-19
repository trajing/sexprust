pub mod integer;
pub mod nil;

pub struct Parse {
    plugins: Vec<Plugin>,
}

impl Parse {
    pub fn parse(&self, s: &str) -> Box<SExpr> {
        println!("TODO!");
        Box::new(nil::Nil)
    }

    pub fn add_plugin_mut(&mut self, p: Plugin) -> Parse {
        self = self.add_plugin(p);
        self
    }

    pub fn add_plugin(&self, p: Plugin) -> Parse {
        println!("TODO!");
    }

    pub fn remove_plugin_mut(&mut self, p: Plugin) -> Parse {
        self = self.remove_plugin(p);
        self
    }

    pub fn remove_plugin(&self, p: Plugin) -> Parse {
        println!("TODO!");
    }

    pub fn default_plugins() -> Parse {
        Parse {
            plugins: vec![integer::IntegerParser] // Put defaults here
        }
    }
}

pub trait PluginData {} // something should go here that we can access..

pub trait Plugin { // is this needed besides AtomParser? Are there any other plugin types?
    fn handle_this(&self, &str) -> bool;
    fn handle(&self, &str) -> PluginData;
}

pub trait AtomParser {
    fn str_is(&self, &str) -> bool;
    fn parse_str<T>(&self, &str) -> AtomData<T=T>;
}

impl Plugin for AtomParser {
    fn handle_this(&self, s: &str) -> bool {
        self.str_is(s)
    }
    fn handle<T>(&self, s: &str) -> AtomData<T=T> {
        self.parse_str(s)
    }
}

pub trait SExpr : PartialEq {}

struct Cons {
    first: SExpr,
    second: SExpr,
}

pub trait AtomData {
    type T : PartialEq;
    fn get_value(&self) -> Self::T;
}

impl PartialEq for Cons {
    fn eq(&self, other: Cons) -> bool {
        self.first == other.first && self.second == other.second
    }
}

impl<T> PartialEq for AtomData<T=T> {
    fn eq(&self, other: AtomData<T=T>) {
        self.get_value() == other.get_value()
    }
}

impl SExpr for Cons {}

impl<T: AtomData> SExpr for T {}
