#[macro_use]
extern crate clap;

use clap::Shell;
use dbus_codegen::{GenOpts, ServerAccess};
use std::fs::{self, File};
use std::io::Write;

include!("src/cli.rs");

fn main() {
    dbus_interface();
    cli();
}

fn dbus_interface() {
    let xml = fs::read_to_string("interface.xml").unwrap();
    let mut dbus_opts = GenOpts::default();
    dbus_opts.serveraccess = ServerAccess::AsRefClosure;
    let interface = dbus_codegen::generate(&xml, &dbus_opts).unwrap();
    let mut out = File::create("src/interface.rs").unwrap();
    out.write_all(&interface.into_bytes()).unwrap();
    println!("cargo:rerun-if-changed=interface.xml");
}

fn cli() {
    let outdir = concat!(env!("CARGO_MANIFEST_DIR"), "/scripts/completion");
    std::fs::create_dir_all(&outdir).unwrap();

    let mut app = build_cli();
    app.gen_completions(crate_name!(), Shell::Bash, &outdir);
    app.gen_completions(crate_name!(), Shell::Elvish, &outdir);
    app.gen_completions(crate_name!(), Shell::Fish, &outdir);
    app.gen_completions(crate_name!(), Shell::Zsh, &outdir);
}
