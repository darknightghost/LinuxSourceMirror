extern crate daemonize;
extern crate json;
#[macro_use]
extern crate log;
#[macro_use]
extern crate structopt;

#[macro_use]
extern crate config_macros;

mod common;
mod config;
mod logger;

use structopt::StructOpt;

#[derive(Debug, structopt::StructOpt)]
#[structopt(
    name = "LinuxSourceMirror",
    about = "Service to create a linux source mirror."
)]
/// Arguments.
struct Arguments {
    /// Config file
    #[structopt(
        short = "c",
        long = "config",
        default_value = "/etc/mirror-server-conf.json"
    )]
    pub config_file: ::std::path::PathBuf,

    /// Run as daemon.
    #[structopt(short = "d", long = "daemon")]
    pub daemon: bool,
}

/// Entery of the application.
fn run_service() -> i32 {
    // Parse arguments.
    let args = Arguments::from_args();
    println!("List of arguments:");
    println!("    {:16} : \"{}\".", "Run as Dameon", args.daemon);
    println!(
        "    {:16} : \"{}\".",
        "Config Path",
        args.config_file.to_str().unwrap()
    );

    // Load config.
    println!("\nLoading config...");
    config::load_config(args.config_file.clone());

    return 0;
}

/// Main function.
fn main() {
    // Run service.
    let exit_code = run_service();

    if exit_code != 0 {
        error!("The service stopped with exit code {}.", exit_code);
    }

    ::std::process::exit(exit_code);
}
