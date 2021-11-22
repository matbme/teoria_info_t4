pub mod components;

use std::env;
use std::fs;
use crate::components::KeyScheduler;
use crate::components::apply_sbox;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let key: &String = &args[2];

    let file_bytes: Vec<u8> = fs::read(filename)
        .expect("Erro ao carregar arquivo");

    let ks = KeyScheduler::new(String::clone(key), 8);
    println!("{:?}", ks);

    let out = apply_sbox(5, 0b011011);
    println!("{}", out);
}
