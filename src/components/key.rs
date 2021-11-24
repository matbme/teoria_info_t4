use crate::components::s_box::AsBitstream;
use crate::components::s_box::FromBitstream;

#[derive(Debug)]
pub struct KeyScheduler {
    pub master_key: [u8; 4],
    pub num_subkeys: usize,
    subkeys: Vec<[u8; 6]>
}

impl KeyScheduler {
    pub fn new(master_key: String, num_subkeys: usize) -> Self {
        let master_key = String::as_bytes(&master_key).clone().try_into()
            .expect("Chave de tamanho inválido. Insira uma chave de 32 bits.");
        let subkeys = Self::gen_keys(&master_key, num_subkeys);
        
        Self { master_key, num_subkeys, subkeys }
    }

    /// Transforms a 32-bit key into a 48-bit key by adding the XOR between
    /// the nibbles of each byte after the original byte.
    ///
    /// # Example:
    ///
    /// let key = "mate"
    ///
    /// Byte 1: "m" => 109 => 01101101 (8 bits)
    /// Out 1 : 01101101 + (0110 ^ 1101) => 011011011011 (12 bits)
    ///
    /// Repeat for each byte and we have a 48-bit key
    fn expand_key(key: &[u8; 4]) -> [u8; 6] {
        let mut ret: [bool; 48] = [false; 48];
        for (i, byte) in key.iter().enumerate() {
            let byte_as_bits = bool::as_bitstream(byte);
            for j in 0..8 {
                ret[i*8+j] = byte_as_bits[j];
            }

            for j in 0..4 {
                ret[i*8+j+8] = byte_as_bits[j] ^ byte_as_bits[4 + j];
            }
        }

        let mut ret_u8: [u8; 6] = [0; 6];
        for (i, byte) in ret.chunks(8).enumerate() {
            ret_u8[i] = u8::from_bitstream(byte.try_into().unwrap());
        }

        ret_u8
    }

    /// Basic sub-key generation. Each sub-key is a byte-wise XOR between the
    /// last sub-key (or the master key if generating the first sub-key) and the
    /// generated key number (between 1 and num_subkeys)
    fn gen_keys(master_key: &[u8; 4], num_subkeys: usize) -> Vec<[u8; 6]> {
        let mut ret: Vec<[u8; 6]> = Vec::new();

        let expanded_masterkey = Self::expand_key(master_key);
        for n in 1..num_subkeys+1 {
            let last_key = if n == 1 { &expanded_masterkey } else { &ret[n-2] };

            let mut new_key: [u8; 6] = [0; 6];
            for (i, b) in last_key.iter().enumerate() {
                new_key[i] = b ^ (n as u8);
            }

            ret.push(new_key);
        }

        ret
    }

    pub fn get_subkey(&self, idx: usize) -> [u8; 6] {
        self.subkeys[idx]
    }
}
