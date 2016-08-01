use std::path::Path;


#[derive(Debug, PartialEq)]
pub enum Status{
    Unknown,
    Compatible,
    MinorBump,
    MajorBump,
}

pub struct Module{
}

pub fn load_mod(root: &Path) -> Module{
    println!("Hm");
    Module{}
}

pub fn compare(a: &Module, b: &Module) -> Status{
    Status::Unknown
}
