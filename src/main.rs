pub mod components;

use std::env;
use std::fs;
use crate::components::*;

fn encrypt(msg: &[bool], ks: &KeyScheduler) -> Vec<bool> {
    let mut ret: Vec<bool> = Vec::new();
    // Division is guaranteed to be integer because of prior padding
    let rounds = msg.iter().len() / 48;
    for i in 0..rounds {
       // Gets a 48 bit chunk
        let mut chunk: [bool; 48] = msg[i*48..i*48+48].try_into().unwrap();

        // XOR with round's subkey
        let round_key = ks.get_subkey(i);
        let round_key = round_key.map(|x| bool::as_bitstream(&x)).concat();

        for (j, bit) in chunk.clone().iter().enumerate() {
            chunk[j] = bit ^ round_key[j];
        }

        // Apply s_box for each byte in chunk
        let mut boxed_chunk = [false; 48];
        for (j, byte) in chunk.chunks(8).enumerate() {
            let boxed = s_box(&u8::from_bitstream(byte.try_into().unwrap()));
            let boxed = bool::as_bitstream(&boxed);

            for k in 0..8 {
                boxed_chunk[j*8+k] = boxed[k];
            }
        }

        // Transforms chunk into two 5x5 matrices
        let mut transformed = block_to_matrix(&boxed_chunk, true);
        // Apply row shift for each matrix
        apply_left_row_shift(&mut transformed[0]);
        apply_left_row_shift(&mut transformed[1]);
        // Transform back to block
        let chunk = matrix_to_block(&transformed, false, true);

        // Add chunk to return vector
        ret.extend_from_slice(&chunk[..]);
    }

    ret
}

fn decrypt(msg: &[bool], ks: &KeyScheduler) -> Vec<bool> {
    let mut ret: Vec<bool> = Vec::new();
    // Unless the file is corrupted, it will always be divisible by 48
    let rounds = msg.iter().len() / 48;
    for i in 0..rounds {
        // Gets a 48 bit chunk
        let chunk: [bool; 48] = msg[i*48..i*48+48].try_into().unwrap();

        // Transforms chunk into two 5x5 matrices
        let mut transformed = block_to_matrix(&chunk, false);
        // Remove row shift for each matrix
        apply_right_row_shift(&mut transformed[0]);
        apply_right_row_shift(&mut transformed[1]);
        // Transform back to block
        let chunk = matrix_to_block(&transformed, false, false);

        // Apply inverse s_box for each byte in chunk
        let mut boxed_chunk = [false; 48];
        for (j, byte) in chunk.chunks(8).enumerate() {
            let boxed = inverse_s_box(&u8::from_bitstream(byte.try_into().unwrap()));
            let boxed = bool::as_bitstream(&boxed);

            for k in 0..8 {
                boxed_chunk[j*8+k] = boxed[k];
            }
        }

        let mut chunk: [bool; 48] = boxed_chunk.clone();

        // XOR with round's subkey
        let round_key = ks.get_subkey(rounds - i - 1);
        let round_key = round_key.map(|x| bool::as_bitstream(&x)).concat();

        for (j, bit) in chunk.clone().iter().enumerate() {
            chunk[j] = bit ^ round_key[j];
        }

        // Add chunk to return vector
        ret.extend_from_slice(&chunk[..]);
        println!("{:?}", ret);
    }

    ret
}

fn main() {
    // Initialize arguments passed by the CLI
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let key: &String = &args[2];

    // Load file into memory
    let mut file_bytes: Vec<u8> = fs::read(filename)
        .expect("Erro ao carregar arquivo");

    file_bytes.pop();

    // Convert to bitstream
    let mut file_bits: Vec<bool> = Vec::new();
    for byte in &file_bytes {
        let mut bitstream: [bool; 8] = [false; 8];
        for b in 0..8 {
            bitstream[b] = (byte & (128 / (2 as u8).pow(b as u32))) != 0;
        }
        file_bits.extend_from_slice(&bitstream[..]);
    }

    // Initialize key scheduler
    let ks = KeyScheduler::new(String::clone(key), 11);

    // Adds padding if needed
    let pad = file_bits.len() % 48;
    for _ in 0..pad {
        file_bits.push(false);
    }

    // Calls encryption
    let enc_msg = encrypt(&file_bits[..], &ks);
    let dec_msg = decrypt(&enc_msg[..], &ks);

    assert_eq!(file_bits, dec_msg);
}
