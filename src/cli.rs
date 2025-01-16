// CLI parsing and sin-command handling

use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("Kizuna")
        .version("0.1.0")
        .author("https://github.com/felixkamau")
        .about("A CLI tool to manage RPC URLs and interact with smart contracts")
        .subcommand(
            Command::new("rpc")
                .about("Manage RPC URLs")
                .subcommand(Command::new("list").about("List all saved RPC URLs"))
                .subcommand(
                    Command::new("add")
                        .about("Add a new RPC URL")
                        .arg(Arg::new("url").help("The RPC URL to add").required(true)),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove an existing RPC URL")
                        .arg(
                            Arg::new("url")
                                .short('r')
                                .long("url")
                                .help("The RPC URL you want to remove")
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            Command::new("contract")
                .about("Interact with a smart contract")
                .arg(
                    Arg::new("rpc-index")
                        .help("Index of the saved RPC URL to use")
                        .long("rpc-index") // Declare it as a long option
                        .value_parser(clap::value_parser!(usize)) // Value parser for usize
                        .default_value("0"), // Default to 0 if not specified
                ),
        )
}
