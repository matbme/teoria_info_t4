pub mod components;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use crate::components::*;

fn cbc(msg: &[bool], last: &[bool]) -> [bool; 48] {
    let mut ret: [bool; 48] = [false; 48];

    for i in 0..48 {
        ret[i] = msg[i] ^ last[i];
    }

    ret
}

fn encrypt(msg: &[bool], ks: &KeyScheduler, iv: &[bool]) -> Vec<bool> {
    let mut ret: Vec<bool> = Vec::new();
    // Division is guaranteed to be integer because of prior padding
    let rounds = msg.iter().len() / 48;
    for i in 0..rounds {
       // Gets a 48 bit chunk
        let chunk: [bool; 48] = msg[i*48..i*48+48].try_into().unwrap();

        // Mix with CBC
        let last_encryption = if i > 0 {
            &ret[(i-1)*48..(i-1)*48+48]
        } else {
            &iv
        };
        let mut chunk = cbc(&chunk, last_encryption);

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
        let chunk = matrix_to_block(&transformed,true);

        // Add chunk to return vector
        ret.extend_from_slice(&chunk[..]);
    }

    ret
}

fn decrypt(msg: &[bool], ks: &KeyScheduler, iv: &[bool]) -> Vec<bool> {
    let mut ret: Vec<bool> = Vec::new();
    let mut dec_chars: Vec<u8> = Vec::new();
    for byte in msg.chunks(8) {
        dec_chars.push(u8::from_bitstream(byte.try_into().unwrap()));
    }
    // println!("{:?}", dec_chars);
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
        let chunk = matrix_to_block(&transformed, false);

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
        let round_key = ks.get_subkey(i);
        let round_key = round_key.map(|x| bool::as_bitstream(&x)).concat();

        for (j, bit) in chunk.clone().iter().enumerate() {
            chunk[j] = bit ^ round_key[j];
        }

        // Unmix with CBC
        let last_encryption = if i > 0 {
            &msg[(i-1)*48..(i-1)*48+48]
        } else {
            &iv
        };
        let chunk = cbc(&chunk, last_encryption);

        // Add chunk to return vector
        ret.extend_from_slice(&chunk[..]);
    }

    ret
}

fn main() {
    // Initialize arguments passed by the CLI
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let key: &String = &args[2];
    let mode: &String = &args[3];

    // Load file into memory
    let mut file_bytes: Vec<u8> = fs::read(filename)
        .expect("Erro ao carregar arquivo");

    file_bytes.pop();

    // Parses mode
    if mode == "encrypt" {
        let iv: [u8; 6] = [18, 83, 254, 123, 173, 164];

        let mut iv_bool: [bool; 48] = [false; 48];
        let mut count = 0;
        for ch in iv {
            for bit in bool::as_bitstream(&ch) {
                iv_bool[count] = bit;
                count += 1;
            }
        }

        // Convert to bitstream
        let mut file_bits: Vec<bool> = Vec::new();
        for byte in &file_bytes {
            let mut bitstream: [bool; 8] = [false; 8];
            for b in 0..8 {
                bitstream[b] = (byte & (128 / (2 as u8).pow(b as u32))) != 0;
            }
            file_bits.extend_from_slice(&bitstream[..]);
        }

        // Adds padding if needed
        let pad = file_bits.len() % 48;
        for _ in 0..pad {
            file_bits.push(false);
        }
        
        // Initialize key scheduler
        let ks = KeyScheduler::new(String::clone(key), file_bits.len() / 48);


        let enc_msg = encrypt(&file_bits[..], &ks, &iv_bool);

        let mut enc_msg_bytes: Vec<u8> = Vec::new();
        for byte in enc_msg.chunks(8) {
            enc_msg_bytes.push(u8::from_bitstream(byte.try_into().unwrap()));
        }


        let mut file = File::create("encrypted.txt").unwrap();
        file.write_all(&iv).expect("Falha ao escrever arquivo");
        file.write_all(&enc_msg_bytes[..]).expect("Falha ao escrever arquivo");
        file.write_all(&[10]).expect("Falha ao escrever arquivo");
    }
    else if mode == "decrypt" {
        let iv: [u8; 6] = file_bytes[..6].try_into().unwrap();

        let mut iv_bool: [bool; 48] = [false; 48];
        let mut count = 0;
        for ch in iv {
            for bit in bool::as_bitstream(&ch) {
                iv_bool[count] = bit;
                count += 1;
            }
        }

        // Convert to bitstream
        let mut file_bits: Vec<bool> = Vec::new();
        for byte in &file_bytes[6..] {
            let mut bitstream: [bool; 8] = [false; 8];
            for b in 0..8 {
                bitstream[b] = (byte & (128 / (2 as u8).pow(b as u32))) != 0;
            }
            file_bits.extend_from_slice(&bitstream[..]);
        }

        // Adds padding if needed
        let pad = file_bits.len() % 48;
        for _ in 0..pad {
            file_bits.push(false);
        }

        // Initialize key scheduler
        let ks = KeyScheduler::new(String::clone(key), file_bits.len() / 48);

        let dec_msg = decrypt(&file_bits[..], &ks, &iv_bool);

        let mut dec_msg_bytes: Vec<u8> = Vec::new();
        for byte in dec_msg.chunks(8) {
            dec_msg_bytes.push(u8::from_bitstream(byte.try_into().unwrap()));
        }
        println!("{:?}", dec_msg_bytes);


        let mut file = File::create("decrypted.txt").unwrap();
        file.write_all(&dec_msg_bytes[..]).expect("Falha ao escrever arquivo");
        file.write_all(&[10]).expect("Falha ao escrever arquivo");
    }
}
