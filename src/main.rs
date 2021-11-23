pub mod components;

use std::env;
use std::fs;
use crate::components::KeyScheduler;
use crate::components::apply_sbox;
use crate::components::block_to_matrix;
use crate::components::matrix_to_block;

fn encrypt(msg: &Vec<u8>, ks: &KeyScheduler) -> Vec<u8> {
    // Division is guaranteed to be integer because of prior padding
   for i in 0..(msg.iter().len() / 48) {
        let chunk: [u8; 48] = msg[..47].try_into().unwrap();
        let transformed = block_to_matrix(&chunk);

        println!("{:?}", chunk);
        println!("{:?}", matrix_to_block(&transformed, false));

    }

    Vec::new()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let key: &String = &args[2];

    let file_bytes: Vec<u8> = fs::read(filename)
        .expect("Erro ao carregar arquivo");

    let mut file_bits: Vec<bool> = Vec::new();

    for byte in &file_bytes {
        let mut bitstream: [bool; 8] = [false; 8];
        for b in 0..8 {
            bitstream[b] = (byte & (128 / (2 as u8).pow(b as u32))) != 0;
        }
        file_bits.extend_from_slice(&bitstream[..]);
    }

    let ks = KeyScheduler::new(String::clone(key), 8);
    println!("{:?}", ks);

    // let out = apply_sbox(5, 0b011011);
    // println!("{}", out);

    let pad = file_bits.len() % 6;
    for _ in 0..pad {
        file_bits.push(false);
    }

    block_to_matrix(&file_bytes[..48].try_into().unwrap());

    encrypt(&file_bytes, &ks);
}
