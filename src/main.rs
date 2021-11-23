pub mod components;

use std::env;
use std::fs;
use crate::components::KeyScheduler;
use crate::components::apply_sbox;
use crate::components::block_to_matrix;

// fn encrypt(msg: &Vec<u8>, ks: &KeyScheduler) -> Vec<u8> {
//     
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let key: &String = &args[2];

    let mut file_bytes: Vec<u8> = fs::read(filename)
        .expect("Erro ao carregar arquivo");

    let ks = KeyScheduler::new(String::clone(key), 8);
    println!("{:?}", ks);

    let out = apply_sbox(5, 0b011011);
    println!("{}", out);

    let pad = file_bytes.len() % 48;
    for _ in 0..pad {
        file_bytes.push(0);
    }

    block_to_matrix(&file_bytes[..48].try_into().unwrap());
}
