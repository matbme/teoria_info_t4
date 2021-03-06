pub fn block_to_matrix(block: &[bool; 48], encrypt: bool) -> [[[bool; 5]; 5]; 2] {
    // Split block into two (2x 24 bits)
    let spl: (&[bool], &[bool]) = block.split_at(24).to_owned();

    let mut ret: [[[bool; 5]; 5]; 2] = [[[false; 5]; 5]; 2];

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

    if !encrypt {
        ret[0][4].rotate_left(4);
        ret[1][4].rotate_left(4);
        // ret[0][4][4] = ret[0][4][0];
        // ret[0][4][0] = false;
        // ret[1][4][4] = ret[1][4][0];
        // ret[1][4][0] = false;
    }

    // Let m be each matrix, m[4][4] corresponds to a XOR between
    // sum([4][..]) and sum([..][4])
    // let row: [u8; 5] = ret[0][4];
    // let col: [u8; 5] = ret[0].map(|r| r[4]);
    // ret[0][4][4] = xor_arr(&row) ^ xor_arr(&col);
    //
    // let row: [u8; 5] = ret[1][4];
    // let col: [u8; 5] = ret[1].map(|r| r[4]);
    // ret[1][4][4] = xor_arr(&row) ^ xor_arr(&col);

    // Applies row shift to both matrices
    // apply_left_row_shift(&mut ret[0]);
    // apply_left_row_shift(&mut ret[1]);

    ret
}

/// Inverse process of the function above
pub fn matrix_to_block(mats: &[[[bool; 5]; 5]; 2], encrypt: bool) -> [bool; 48] {
    let lhs = mats[0];
    let rhs = mats[1];

    let mut ret: Vec<bool> = Vec::new();

    let pos = if encrypt { 20 } else { 24 };
    for i in 0..5 {
        for j in 0..5 {
            if (i*5+j) != pos {
                ret.push(lhs[i][j]);
            }
        }
    }

    for i in 0..5 {
        for j in 0..5 {
            if (i*5+j) != pos {
                ret.push(rhs[i][j]);
            }
        }
    }

    ret.try_into().unwrap()
}

/// Applies row shift for a 5x5 matrix
pub fn apply_left_row_shift(mat: &mut [[bool; 5]; 5]) {
    for i in 0..mat.len() {
        let result = mat.get_mut(i);
        match result {
            Some(x) => x.rotate_left(i),
            None    => println!("Erro ao realizar a op. de row shift"),
        }
    }
}


/// Removes row shift for a 5x5 matrix
pub fn apply_right_row_shift(mat: &mut [[bool; 5]; 5]) {
    for i in 0..mat.len() {
        let result = mat.get_mut(i);
        match result {
            Some(x) => x.rotate_right(i),
            None    => println!("Erro ao realizar a op. de row shift"),
        }
    }
}
