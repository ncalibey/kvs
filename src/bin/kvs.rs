use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Command {
    Get {
        #[structopt()]
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
    Rm {
        key: String,
    },
}

#[derive(StructOpt, Debug)]
struct App {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

fn main() {
    exit(match run_app() {
        Err(code) => code,
        _ => 0,
    })
}

fn run_app() -> Result<(), i32> {
    let app = App::from_args();
    match app.cmd {
        Some(_) => {
            eprintln!("unimplemented");
        }
        _ => (),
    }
    Err(1)
}
