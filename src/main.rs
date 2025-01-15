use cli::build_cli;
use rpc::handle_rpc_subcommand;
use storage::Storage;

mod cli;
mod storage;
mod rpc;


fn main() {
    let matches = build_cli().get_matches();

    let mut storage = Storage::load();

    if let Some(rpc_matches) = matches.subcommand_matches("rpc") {
        handle_rpc_subcommand(rpc_matches, &mut storage);
    }else {
        println!("Unknown command. Use --help for more information.");
    }
}
