pub fn block_to_matrix(block: &[u8; 48]) -> [[[u8; 5]; 5]; 2] {
    // Split block into two (2x 24 bits)
    let spl: (&[u8], &[u8]) = block.split_at(24).to_owned();

    let mut ret: [[[u8; 5]; 5]; 2] = [[[0; 5]; 5]; 2];

    // Transform each block into a 5x5 matrix
    let mut i = 0;
    for v in spl.0 {
        ret[0][(i / 5) as usize][i % 5] = v.clone();
        i += 1;
    }
    i = 0;
    for v in spl.1 {
        ret[1][(i / 5) as usize][i % 5] = v.clone();
        i += 1;
    }

    // Let m be each matrix, m[4][4] corresponds to a XOR between
    // sum([4][..]) and sum([..][4])
    let row: [u8; 5] = ret[0][4];
    let col: [u8; 5] = ret[0].map(|r| r[4]);
    ret[0][4][4] = xor_arr(&row) ^ xor_arr(&col);

    let row: [u8; 5] = ret[1][4];
    let col: [u8; 5] = ret[1].map(|r| r[4]);
    ret[1][4][4] = xor_arr(&row) ^ xor_arr(&col);

    // Applies row shift to both matrices
    apply_row_shift(&mut ret[0]);
    apply_row_shift(&mut ret[1]);

    ret
}

/// XOR's a 5 element array
fn xor_arr(arr: &[u8; 5]) -> u8 {
    let mut ret: u8 = 0;

    for elem in arr {
        ret ^= elem;
    }

    ret
}


/// Applies row shift for a 5x5 matrix
fn apply_row_shift(mat: &mut [[u8; 5]; 5]) {
    for i in 0..mat.len() {
        let result = mat.get_mut(i);
        match result {
            Some(x) => x.rotate_left(i),
            None    => println!("Erro ao realizar a op. de row shift"),
        }
    }
}
