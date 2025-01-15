
use clap::ArgMatches;

use crate::storage::Storage;


pub fn handle_rpc_subcommand(matches: &ArgMatches, storage: &mut Storage) {
    if matches.subcommand_matches("list").is_some(){
        list_rpc_urls(storage);
    }else if let Some(add_matches) =  matches.subcommand_matches("add"){
        if let Some(url) = add_matches.get_one::<String>("url"){
            add_rpc_url(url, storage);
        }
    }else if let Some(remove_matches) = matches.subcommand_matches("remove"){
        if let Some(url) = remove_matches.get_one::<String>("url"){
            remove_rpc_url(url, storage);
        }
    }
}

fn list_rpc_urls(storage: &Storage) {
    println!("Saved RPC URLs:");
    for (i, url) in storage.rpc_urls.iter().enumerate() {
        println!("{}: {}", i + 1, url);
    }
}

fn add_rpc_url(url: &str, storage: &mut Storage) {
    if !storage.rpc_urls.contains(&url.to_string()) {
        storage.rpc_urls.push(url.to_string());
        storage.save();
        println!("Added RPC URL: {}", url);
    } else {
        println!("RPC URL already exists: {}", url);
    }
}

fn remove_rpc_url(url: &str, storage: &mut Storage) {
    if let Some(index) = storage.rpc_urls.iter().position(|x| x == url) {
        storage.rpc_urls.remove(index);
        storage.save();
        println!("Removed RPC URL: {}", url);
    } else {
        println!("RPC URL not found: {}", url);
    }
}