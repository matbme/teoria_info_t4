pub fn block_to_matrix(block: &[u8; 48]) -> [[[u8; 5]; 5]; 2] {
    let spl: (&[u8], &[u8]) = block.split_at(24).to_owned();
    let mut ret: [[[u8; 5]; 5]; 2] = [[[0; 5]; 5]; 2];

    let mut i = 0;
    for v in spl.0 {
        ret[0][(i / 5) as usize][i % 5] = v.clone();
        i += 1;
    }

    println!("{:?}", ret[0]);

    ret
}
