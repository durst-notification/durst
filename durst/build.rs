use dbus_codegen::{generate, GenOpts};
use std::fs::{self, File};
use std::io::Write;

fn main() {
    let xml = fs::read_to_string("interface.xml").unwrap();
    let interface = generate(&xml, &GenOpts::default()).unwrap();
    let mut out = File::create("src/interface.rs").unwrap();
    out.write_all(&interface.into_bytes()).unwrap();
    println!("cargo:rerun-if-changed=interface.xml");
}
