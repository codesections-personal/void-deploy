use clap::{crate_name, crate_version, App, Arg, ArgMatches};
use std::error::Error;
use utils::{sh, Die};

fn main() {
    #[rustfmt::skip]
    let cli = App::new(crate_name!())
        .version(crate_version!())
        .about("Builds and strips a Rust binary and then copies it to my Void laptop")
        .arg(Arg::with_name("BINARY")
             .required_unless("src")
             .takes_value(true)
             .help("The name of the binary to build/deploy"))
        .arg(Arg::from("--src 'Prints this program's source to stdout'"))
        .get_matches();
    run(cli).unwrap_or_die();
}

fn run(cli: ArgMatches) -> Result<(), Box<dyn Error>> {
    if cli.is_present("src") {
        print!("/// main.rs\n{}", include_str!("main.rs"));
    } else {
        let (out, err) = sh(&format!(
            r"
cargo build --release
strip target/release/{name}
scp target/release/{name} T480:~/bin/
",
            name = cli.value_of("BINARY").expect("required by clap")
        ))?;
        print!("{}{}", out, err);
    }
    Ok(())
}
