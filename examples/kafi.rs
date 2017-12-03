


extern crate kafi;


use std::env;

use kafi::Store;



fn main() {

    let args:Vec<String> = env::args().collect();

    let mut store:Store<String, String> = Store::open("kafi.db").unwrap();

    match args.len() {
        2 => {
            if let Some(v) = store.get(&args[1]) {
                println!("{}", v);
            }
        },
        3 => {
            store.insert(args[1].to_string(), args[2].to_string());
        },
        _ => print_usage()
    }
}


fn print_usage(){
    println!("USAGE: ");
    println!("   to set: kafi [KEY] [VALUE]");
    println!("   to get: kafi [KEY]");
}
