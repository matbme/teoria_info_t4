pub trait AsBitstream<T>: Sized {
    fn as_bitstream(val: &T) -> [Self; 8];
}

pub trait FromBitstream {
    fn from_bitstream(val: &[bool; 8]) -> Self;
}

impl AsBitstream<u8> for bool {
    fn as_bitstream(val: &u8) -> [bool; 8] {
        let mut bitstream: [bool; 8] = [false; 8];
        for b in 0..8 {
            bitstream[b] = (val & (128 / (2 as u8).pow(b as u32))) != 0;
        }

        bitstream
    }
}

impl FromBitstream for u8 {
    fn from_bitstream(val: &[bool; 8]) -> u8 {
        let mut ret: u8 = 0;

        for (i, bit) in val.iter().enumerate() {
            ret += (*bit as u8) * (2 as u8).pow((7 - i) as u32);
        }

        ret
    }
}

pub fn s_box(num: &u8) -> u8 {
    let mut confusion_matrix: [[bool; 8]; 8] = [[false; 8]; 8];

    // build confusion matrix
    for i in 0..8 {
        let mut row = bool::as_bitstream(&0x1F);
        row.rotate_left(7-i);
        confusion_matrix[i] = row;
    }

    // reverse number
    let mut inv = bool::as_bitstream(num);
    inv.reverse();

    // runs through matrix
    let mut oper = [false; 8];
    let mut out = [false; 8];
    for (i, row) in confusion_matrix.iter().enumerate() {
        for (j, bit) in inv.iter().rev().enumerate() {
            oper[j] = bit & row[j];
        }

        let mut xor_result = oper[0];
        for bit in oper[1..].iter() {
            xor_result ^= bit;
        }

        out[i] = xor_result;
    }

    // reverse output and XOR with final vector
    out.reverse();
    let out: Vec<bool> = out.iter().zip(bool::as_bitstream(&(0x63 as u8)))
        .map(|(x, y)| x ^ y).collect();

    // back to array
    let mut out_arr = [false; 8];
    for i in 0..8 {
        out_arr[i] = out[i];
    }

    println!("{:x}, {:x}", num, u8::from_bitstream(&out_arr));

    0x10 as u8
}
