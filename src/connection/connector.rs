#[derive(Debug, PartialEq)]
pub struct Connector;

impl Connector {
    pub fn new() -> Self {
        println!("My constructor was called!");
        Self
    }
}
