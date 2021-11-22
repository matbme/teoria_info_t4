pub mod encrypt;

use std::env;
use std::fs;
use crate::encrypt::KeyScheduler;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let key: &String = &args[2];

    let file_bytes: Vec<u8> = fs::read(filename)
        .expect("Erro ao carregar arquivo");

    let ks = KeyScheduler::new(String::clone(key), 4);
    println!("{:?}", ks);
}
