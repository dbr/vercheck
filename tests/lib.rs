extern crate vercheck;

use std::path::Path;


#[test]
fn same_api() {
    let a = vercheck::load_mod(Path::new("a_v1.0.0/"));
    let b = vercheck::load_mod(Path::new("a_v1.0.0/"));
    let status = vercheck::compare(&a, &b);
    assert!(status == vercheck::Status::Compatible);
}

#[test]
fn minor_bump() {
    let a = vercheck::load_mod(Path::new("a_v1.0.0/"));
    let b = vercheck::load_mod(Path::new("a_v1.1.0/"));
    let status = vercheck::compare(&a, &b);
    assert!(status == vercheck::Status::MinorBump);
}

#[test]
fn major_bump() {
    let a = vercheck::load_mod(Path::new("a_v1.1.0/"));
    let b = vercheck::load_mod(Path::new("a_v2.0.0/"));
    let status = vercheck::compare(&a, &b);
    assert!(status == vercheck::Status::MajorBump);
}
