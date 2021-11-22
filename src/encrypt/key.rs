#[derive(Debug)]
pub struct KeyScheduler {
    pub master_key: Vec<u8>,
    pub num_subkeys: usize,
    subkeys: Vec<Vec<u8>>
}

impl KeyScheduler {
    pub fn new(master_key: String, num_subkeys: usize) -> Self {
        let master_key = String::as_bytes(&master_key).to_vec();
        let subkeys = Self::gen_keys(&master_key, num_subkeys);
        
        Self { master_key, num_subkeys, subkeys }
    }

    /// Basic sub-key generation. Each sub-key is a byte-wise XOR between the
    /// last sub-key (or the master key if generating the first sub-key) and the
    /// generated key number (between 1 and num_subkeys)
    fn gen_keys(master_key: &Vec<u8>, num_subkeys: usize) -> Vec<Vec<u8>> {
        let mut ret: Vec<Vec<u8>> = Vec::new();

        for n in 1..num_subkeys {
            println!("Doing for {}", n);
            let last_key = if n == 1 { master_key } else { &ret[n-2] };

            let mut new_key: Vec<u8> = Vec::new();
            for b in last_key.iter() {
                new_key.push(b ^ (n as u8));
            }

            ret.push(new_key);
        }

        ret
    }
}
