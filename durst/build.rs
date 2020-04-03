use dbus_codegen::{generate, GenOpts, ServerAccess};
use std::fs::{self, File};
use std::io::Write;

fn main() {
    let xml = fs::read_to_string("interface.xml").unwrap();
    let mut options = GenOpts::default();
    options.serveraccess = ServerAccess::AsRefClosure;
    let interface = generate(&xml, &options).unwrap();
    let mut out = File::create("src/interface.rs").unwrap();
    out.write_all(&interface.into_bytes()).unwrap();
    println!("cargo:rerun-if-changed=interface.xml");
}
