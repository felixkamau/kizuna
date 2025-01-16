use crate::storage::Storage;
use clap::ArgMatches;

pub fn handle_contract_subcommand(matches: &ArgMatches, storage: &Storage) {
    // Extract the rpc-index argument as usize
    if let Some(rpc_index) = matches.get_one::<usize>("rpc-index") {
        // Now rpc_index is directly a usize, so no need to parse it
        if let Some(rpc_url) = storage.get_rpc_url(*rpc_index) {
            println!("Using RPC URL: {}", rpc_url);
            // Here you can use the RPC URL to interact with the smart contract
            // Example: interact_with_contract(rpc_url);
        } else {
            println!("RPC URL not found at index {}", rpc_index);
        }
    } else {
        println!("No rpc-index provided or invalid value.");
    }
}


// pub fn handle_contract_subcommand(matches: &ArgMatches, storage: &Storage) {
//     // Extract the rpc-index argument from the contract subcommand
//     if let Some(rpc_index_str) = matches.get_one::<String>("rpc-index") {
//         let rpc_index: usize = rpc_index_str.parse().unwrap_or(0); // Default to 0 if parsing fails

//         // Get the RPC URL using the index
//         if let Some(rpc_url) = storage.get_rpc_url(rpc_index) {
//             println!("Using RPC URL: {}", rpc_url);
//             // Here you can use the RPC URL to interact with the smart contract
//             // Example: interact_with_contract(rpc_url);
//         } else {
//             println!("RPC URL not found at index {}", rpc_index);
//         }
//     } else {
//         // If the rpc-index isn't provided, handle it accordingly
//         println!("No rpc-index provided, using default (0).");
//         if let Some(rpc_url) = storage.get_rpc_url(0) {
//             println!("Using default RPC URL: {}", rpc_url);
//             // Example: interact_with_contract(rpc_url);
//         } else {
//             println!("RPC URL not found at default index (0).");
//         }
//     }
// }
