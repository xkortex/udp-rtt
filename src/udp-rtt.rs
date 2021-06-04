extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("udp-rtt")
        .version("0.1")
        .author("Mike M")
        .about("Determine round-trip time and offset between hosts")
        .args_from_usage(
            "-c, --config=[FILE] 'Sets a custom config file'
                              -v...                'Sets the level of verbosity'")
        .subcommand(SubCommand::with_name("client")
            .about("Runs the client")
            .arg_from_usage("-d, --debug 'Print debug information'")
        ).subcommand(SubCommand::with_name("help")
            .about("helps")
            .arg_from_usage("-d, --debug 'Print debug information'
            ")
        )
        .get_matches();

    println!("{:?}", matches)
}