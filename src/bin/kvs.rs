use clap::{App, Arg, SubCommand};
use std::env;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kvs")]
struct Opt {}

fn main() {
    exit(match run_app() {
        Err(code) => code,
        _ => 0,
    })
}

fn run_app() -> Result<(), i32> {
    let matches = App::new("kvs")
        .version(env::var_os("CARGO_PKG_VERSION").unwrap().to_str().unwrap())
        .author("Nick Calibey <nick.calibey@gmail.com>")
        .about("Key-value store")
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("key").value_name("KEY").required(true))
                .arg(Arg::with_name("value").value_name("VALUE").required(true)),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("key").value_name("KEY").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("key").value_name("KEY").required(true)),
        )
        .get_matches();
    if let Some(_) = matches.subcommand_matches("get") {
        eprintln!("unimplemented");
        return Err(1);
    }
    if let Some(_) = matches.subcommand_matches("set") {
        eprintln!("unimplemented");
        return Err(1);
    }
    if let Some(_) = matches.subcommand_matches("rm") {
        eprintln!("unimplemented");
        return Err(1);
    }
    if matches.args.len() == 0 {
        return Err(1);
    }
    Ok(())
}
