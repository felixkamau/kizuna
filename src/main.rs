use cli::build_cli;
use contract::handle_contract_subcommand;
use rpc::handle_rpc_subcommand;
use storage::Storage;

mod cli;
mod contract;
mod rpc;
mod storage;

fn main() {
    let matches = build_cli().get_matches();

    let mut storage = Storage::load();

    if let Some(rpc_matches) = matches.subcommand_matches("rpc") {
        handle_rpc_subcommand(rpc_matches, &mut storage);
    } else if let Some(contract_matches) = matches.subcommand_matches("contract") {
        handle_contract_subcommand(contract_matches, &storage);
    } else {
        println!("Unknown command. Use --help for more information.");
    }
}
