#![allow(dead_code)]
#![feature(const_fn)]
#![feature(const_panic)]
#![feature(const_eval_limit)]
#![const_eval_limit = "14000000"]

use std::env;
use std::fmt::LowerHex;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

mod magics;

const ROOK_DELTAS: [i32; 4] = [8, 1, -8, -1];
const BISHOP_DELTAS: [i32; 4] = [9, 7, -9, -7];
const KING_DELTAS: [i32; 8] = [9, 8, 7, 1, -9, -8, -7, -1];
const KNIGHT_DELTAS: [i32; 8] = [17, 15, 10, 6, -17, -15, -10, -6];
const WHITE_PAWN_DELTAS: [i32; 2] = [7, 9];
const BLACK_PAWN_DELTAS: [i32; 2] = [-7, -9];

const fn sliding_attacks(square: i32, occupied: u64, deltas: &[i32]) -> u64 {
    let mut attack = 0;

    let mut i = 0;
    let len = deltas.len();
    while i < len {
        let mut previous = square;
        loop {
            let sq = previous + deltas[i];
            let file_diff = (sq & 0x7) - (previous & 0x7);
            if file_diff > 2 || file_diff < -2 || sq < 0 || sq > 63 {
                break;
            }
            let bb = 1 << sq;
            attack |= bb;
            if occupied & bb != 0 {
                break;
            }
            previous = sq;
        }
        i += 1;
    }

    attack
}

const fn init_stepping_attacks(deltas: &[i32]) -> [u64; 64] {
    let mut table = [0; 64];
    let mut sq = 0;
    while sq < 64 {
        table[sq] = sliding_attacks(sq as i32, !0, deltas);
        sq += 1;
    }
    table
}

pub const KNIGHT_ATTACKS: [u64; 64] = init_stepping_attacks(&KNIGHT_DELTAS);
pub const KING_ATTACKS: [u64; 64] = init_stepping_attacks(&KING_DELTAS);
pub const WHITE_PAWN_ATTACKS: [u64; 64] = init_stepping_attacks(&WHITE_PAWN_DELTAS);
pub const BLACK_PAWN_ATTACKS: [u64; 64] = init_stepping_attacks(&BLACK_PAWN_DELTAS);

const fn init_rays() -> [[u64; 64]; 64] {
    let mut table = [[0; 64]; 64];
    let mut a = 0;
    while a < 64 {
        let mut b = 0;
        while b < 64 {
            table[a as usize][b as usize] = if a == b {
                0
            } else if a & 7 == b & 7 {
                0x0101_0101_0101_0101 << (a & 7)
            } else if a >> 3 == b >> 3 {
                0xff << 8 * (a >> 3)
            } else {
                let diag = (a >> 3) - (a & 7);
                let anti_diag = (a >> 3) + (a & 7) - 7;
                if diag == (b >> 3) - (b & 7) {
                    if diag >= 0 {
                        0x8040_2010_0804_0201 << (8 * diag)
                    } else {
                        0x8040_2010_0804_0201 >> (8 * -diag)
                    }
                } else if anti_diag == (b >> 3) + (b & 7) - 7 {
                    if anti_diag >= 0 {
                        0x0102_0408_1020_4080 << (8 * anti_diag)
                    } else {
                        0x0102_0408_1020_4080 >> (8 * -anti_diag)
                    }
                } else {
                    0
                }
            };
            b += 1;
        }
        a += 1;
    }
    table
}

pub const RAYS: [[u64; 64]; 64] = init_rays();

const fn init_magics() -> [u64; 88772] {
    let mut table = [0; 88772];
    let mut square = 0;
    while square < 64 {
        let magic = &magics::BISHOP_MAGICS[square as usize];
        let range = magic.mask;
        let mut subset = 0;
        loop {
            let attack = sliding_attacks(square, subset, &BISHOP_DELTAS);
            let idx = (magic.factor.wrapping_mul(subset) >> (64 - 9)) as usize + magic.offset;
            assert!(table[idx] == 0 || table[idx] == attack);
            table[idx] = attack;
            subset = subset.wrapping_sub(range) & range;
            if subset == 0 {
                break;
            }
        }

        let magic = &magics::ROOK_MAGICS[square as usize];
        let range = magic.mask;
        let mut subset = 0;
        loop {
            let attack = sliding_attacks(square, subset, &ROOK_DELTAS);
            let idx = (magic.factor.wrapping_mul(subset) >> (64 - 12)) as usize + magic.offset;
            assert!(table[idx] == 0 || table[idx] == attack);
            table[idx] = attack;
            subset = subset.wrapping_sub(range) & range;
            if subset == 0 {
                break;
            }
        }

        square += 1;
    }
    table
}

pub const ATTACKS: [u64; 88772] = init_magics();

fn dump_slice<W: Write, T: Clone + LowerHex>(w: &mut W, name: &str, tname: &str, slice: &[T]) -> io::Result<()> {
    writeln!(w, "#[allow(clippy::unreadable_literal)]")?;
    write!(w, "static {}: [{}; {}] = [", name, tname, slice.len())?;
    for v in slice.iter().cloned() {
        write!(w, "0x{:x}, ", v)?;
    }
    writeln!(w, "];")
}

fn dump_table<W: Write, T: Clone + LowerHex>(w: &mut W, name: &str, tname: &str, table: &[[T; 64]; 64]) -> io::Result<()> {
    writeln!(w, "#[allow(clippy::unreadable_literal)]")?;
    write!(w, "static {}: [[{}; 64]; 64] = [", name, tname)?;
    for row in table.iter() {
        write!(w, "[")?;
        for column in row.iter().cloned() {
            write!(w, "0x{:x}, ", column)?;
        }
        write!(w, "], ")?;
    }
    writeln!(w, "];")
}

fn main() -> io::Result<()> {
    // generate attacks.rs
    let out_dir = env::var("OUT_DIR").expect("got OUT_DIR");
    let dest_path = Path::new(&out_dir).join("attacks.rs");
    let mut f = File::create(&dest_path).expect("created attacks.rs");

    dump_slice(&mut f, "KNIGHT_ATTACKS", "u64", &KNIGHT_ATTACKS)?;
    dump_slice(&mut f, "KING_ATTACKS", "u64", &KING_ATTACKS)?;
    dump_slice(&mut f, "WHITE_PAWN_ATTACKS", "u64", &WHITE_PAWN_ATTACKS)?;
    dump_slice(&mut f, "BLACK_PAWN_ATTACKS", "u64", &BLACK_PAWN_ATTACKS)?;

    writeln!(f)?;

    dump_table(&mut f, "RAYS", "u64", &RAYS)?;

    writeln!(f)?;

    dump_slice(&mut f, "ATTACKS", "u64", &ATTACKS)
}
