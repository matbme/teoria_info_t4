#[derive(Debug)]
pub struct KeyScheduler {
    pub master_key: [u8; 4],
    pub num_subkeys: usize,
    subkeys: Vec<[u8; 4]>
}

impl KeyScheduler {
    pub fn new(master_key: String, num_subkeys: usize) -> Self {
        let master_key = String::as_bytes(&master_key).clone().try_into()
            .expect("Chave de tamanho inválido. Insira uma chave de 32 bits.");
        let subkeys = Self::gen_keys(&master_key, num_subkeys);
        
        Self { master_key, num_subkeys, subkeys }
    }

    /// Basic sub-key generation. Each sub-key is a byte-wise XOR between the
    /// last sub-key (or the master key if generating the first sub-key) and the
    /// generated key number (between 1 and num_subkeys)
    fn gen_keys(master_key: &[u8; 4], num_subkeys: usize) -> Vec<[u8; 4]> {
        let mut ret: Vec<[u8; 4]> = Vec::new();

        for n in 1..num_subkeys {
            let last_key = if n == 1 { master_key } else { &ret[n-2] };

            let mut new_key: [u8; 4] = [0; 4];
            for (i, b) in last_key.iter().enumerate() {
                new_key[i] = b ^ (n as u8);
            }

            ret.push(new_key);
        }

        ret
    }

    pub fn get_subkey(&self, idx: u8) -> [u8; 4] {
        self.subkeys[idx as usize]
    }
}
