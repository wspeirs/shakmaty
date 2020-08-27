#![allow(dead_code)]
#![feature(const_eval_limit)]
#![const_eval_limit = "20000000"]

use std::env;
use std::fmt::LowerHex;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

mod errors;
mod types;
mod square;
mod bitboard;
mod magics;

use crate::square::Square;
use crate::bitboard::Bitboard;
use crate::magics::Magic;

const ROOK_DELTAS: [i32; 4] = [8, 1, -8, -1];
const BISHOP_DELTAS: [i32; 4] = [9, 7, -9, -7];
const KING_DELTAS: [i32; 8] = [9, 8, 7, 1, -9, -8, -7, -1];
const KNIGHT_DELTAS: [i32; 8] = [17, 15, 10, 6, -17, -15, -10, -6];
const WHITE_PAWN_DELTAS: [i32; 2] = [7, 9];
const BLACK_PAWN_DELTAS: [i32; 2] = [-7, -9];

const fn better_sliding_attacks(square: i32, occupied: u64, deltas: &[i32]) -> u64 {
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

const fn tabulate_stepping_attack(deltas: &[i32]) -> [u64; 64] {
    let mut table = [0; 64];
    let mut sq = 0;
    while sq < 64 {
        table[sq] = better_sliding_attacks(sq as i32, !0, deltas);
        sq += 1;
    }
    table
}

static KNIGHT_ATTACKS: [u64; 64] = tabulate_stepping_attack(&KNIGHT_DELTAS);
const KING_ATTACKS: [u64; 64] = tabulate_stepping_attack(&KING_DELTAS);
const WHITE_PAWN_ATTACKS: [u64; 64] = tabulate_stepping_attack(&WHITE_PAWN_DELTAS);
const BLACK_PAWN_ATTACKS: [u64; 64] = tabulate_stepping_attack(&BLACK_PAWN_DELTAS);

const fn tabulate_rays() -> [[u64; 64]; 64] {
    let mut table = [[0; 64]; 64];
    let mut a = 0;
    while a < 64 {
        let mut b = 0;
        while b < 64 {
            table[a][b] = if a == b {
                0
            } else if a & 7 == b & 7 {
                1 << (a & 7)
            } else if a >> 3 == b >> 3 {
                0
            } else {
                0
            };
            b += 1;
        }
        a += 1;
    }
    table
}

const RAYS: [[u64; 64]; 64] = tabulate_rays();

const fn better_init_magics() -> [u64; 88772] {
    let mut table = [0; 88772];
    let mut square = 0;
    while square < 64 {
        let range = better_sliding_attacks(square, 0, &BISHOP_DELTAS);
        let mut subset = 0;
        loop {
            let attack = better_sliding_attacks(square, subset, &BISHOP_DELTAS);
            let magic = &magics::BISHOP_MAGICS[square as usize];
            let idx = (magic.factor.wrapping_mul(subset) >> (64 - 9)) as usize + magic.offset;
            //assert!(table[idx] == 0 || table[idx] == attack);
            table[idx] = attack;
            subset = subset.wrapping_sub(range) & range;
            if subset == 0 {
                break;
            }
        }

        /* let range = better_sliding_attacks(square, 0, &ROOK_DELTAS);
        let mut subset = 0;
        loop {
            let attack = better_sliding_attacks(square, subset, &ROOK_DELTAS);
            let magic = &magics::ROOK_MAGICS[square as usize];
            let idx = (magic.factor.wrapping_mul(subset) >> (64 - 12)) as usize + magic.offset;
            //assert!(table[idx] == 0 || table[idx] == attack);
            table[idx] = attack;
            subset = subset.wrapping_sub(range) & range;
            if subset == 0 {
                break;
            }
        } */

        square += 1;
    }
    table
}

const ATTACKS: [u64; 88772] = better_init_magics();

fn sliding_attacks(sq: Square, occupied: Bitboard, deltas: &[i32]) -> Bitboard {
    let mut attack = Bitboard(0);

    for delta in deltas {
        let mut previous = sq;

        while let Some(s) = previous.offset(*delta) {
            if s.distance(previous) > 2 {
                break;
            }

            attack.add(s);

            if occupied.contains(s) {
                break;
            }

            previous = s;
        }
    }

    attack
}

fn sliding_bishop_attacks(sq: Square, occupied: Bitboard) -> Bitboard {
    sliding_attacks(sq, occupied, &BISHOP_DELTAS)
}

fn sliding_rook_attacks(sq: Square, occupied: Bitboard) -> Bitboard {
    sliding_attacks(sq, occupied, &ROOK_DELTAS)
}

fn step_attacks(sq: Square, deltas: &[i32]) -> Bitboard {
    sliding_attacks(sq, Bitboard::ALL, deltas)
}

fn init_magics(sq: Square, magic: &Magic, shift: u32, attacks: &mut [Bitboard], deltas: &[i32]) {
    for subset in Bitboard(magic.mask).carry_rippler() {
        let attack = sliding_attacks(sq, subset, deltas);
        let idx = (magic.factor.wrapping_mul(subset.0) >> (64 - shift)) as usize + magic.offset;
        assert!(attacks[idx].is_empty() || attacks[idx] == attack);
        attacks[idx] = attack;
    }
}

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
    generate_basics(&mut f)?;
    generate_sliding_attacks(&mut f)
}

fn generate_basics<W: Write>(f: &mut W) -> io::Result<()> {
    let mut knight_attacks = [Bitboard(0); 64];
    let mut king_attacks = [Bitboard(0); 64];
    let mut white_pawn_attacks = [Bitboard(0); 64];
    let mut black_pawn_attacks = [Bitboard(0); 64];

    let mut bb_rays = [[Bitboard(0); 64]; 64];

    for sq in Bitboard::ALL {
        knight_attacks[usize::from(sq)] = step_attacks(sq, &KNIGHT_DELTAS);
        king_attacks[usize::from(sq)] = step_attacks(sq, &KING_DELTAS);
        white_pawn_attacks[usize::from(sq)] = step_attacks(sq, &WHITE_PAWN_DELTAS);
        black_pawn_attacks[usize::from(sq)] = step_attacks(sq, &BLACK_PAWN_DELTAS);
    }

    for a in Bitboard::ALL {
        for b in sliding_bishop_attacks(a, Bitboard(0)) {
            bb_rays[usize::from(a)][usize::from(b)] =
                (sliding_bishop_attacks(a, Bitboard(0)) &
                 sliding_bishop_attacks(b, Bitboard(0))).with(a).with(b);
        }
        for b in sliding_rook_attacks(a, Bitboard(0)) {
            bb_rays[usize::from(a)][usize::from(b)] =
                (sliding_rook_attacks(a, Bitboard(0)) &
                 sliding_rook_attacks(b, Bitboard(0))).with(a).with(b);
        }
    }

    dump_slice(f, "KNIGHT_ATTACKS", "u64", &knight_attacks)?;
    dump_slice(f, "KING_ATTACKS", "u64", &king_attacks)?;
    dump_slice(f, "WHITE_PAWN_ATTACKS", "u64", &white_pawn_attacks)?;
    dump_slice(f, "BLACK_PAWN_ATTACKS", "u64", &black_pawn_attacks)?;

    writeln!(f)?;

    dump_table(f, "BB_RAYS", "u64", &bb_rays)?;

    writeln!(f)
}

fn generate_sliding_attacks<W: Write>(f: &mut W) -> io::Result<()> {
    let mut attacks = [Bitboard(0); 88772];

    for sq in Bitboard::ALL {
        init_magics(sq, &magics::ROOK_MAGICS[usize::from(sq)], 12, &mut attacks, &ROOK_DELTAS);
        init_magics(sq, &magics::BISHOP_MAGICS[usize::from(sq)], 9, &mut attacks, &BISHOP_DELTAS);
    }

    dump_slice(f, "ATTACKS", "u64", &attacks)
}
