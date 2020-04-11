use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("verbose")
                .help("turn on debugging information")
                .long("verbose")
                .short("v"),
        )
        .arg(
            Arg::with_name("config-path")
                .value_name("FILE")
                .help("Use alternative config file")
                .long("config")
                .short("c")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("mode")
                .help("Overwrite the automatic output-mode")
                .long("force-output")
                .short("o")
                .takes_value(true)
                .possible_values(&["wayland", "xorg", "stdout"]),
        )
}
