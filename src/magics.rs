// Fixed shift white magics found by Volker Annuss.
// From: http://www.talkchess.com/forum/viewtopic.php?p=727500&t=64790

pub struct Magic {
    pub mask: u64,
    pub factor: u64,
    pub offset: usize,
}

#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
pub static ROOK_MAGICS: [Magic; 64] = [
    Magic { mask: 0x7e01010101010100, factor: 0xebffffb9ff9fc526, offset: 30727 }, // sq: 56
    Magic { mask: 0x7c02020202020200, factor: 0x100112010e401486, offset: 32775 }, // sq: 57
    Magic { mask: 0x7a04040404040400, factor: 0xb0404a2000470901, offset: 34822 }, // sq: 58
    Magic { mask: 0x7608080808080800, factor: 0x10a00a0018400c86, offset: 36869 }, // sq: 59
    Magic { mask: 0x6e10101010101000, factor: 0x9110008003a1085, offset: 38915 }, // sq: 60
    Magic { mask: 0x5e20202020202000, factor: 0x5009208040005, offset: 40963 }, // sq: 61
    Magic { mask: 0x3e40404040404000, factor: 0x2d28087008a21094, offset: 43011 }, // sq: 62
    Magic { mask: 0x7e80808080808000, factor: 0x7645fffecbfea79e, offset: 45058 }, // sq: 63
    Magic { mask: 0x7e010101010100, factor: 0xf0a5009e00b02a00, offset: 20489 }, // sq: 48
    Magic { mask: 0x7c020202020200, factor: 0x9aa0001004080010, offset: 22536 }, // sq: 49
    Magic { mask: 0x7a040404040400, factor: 0x8c8040200100010, offset: 23560 }, // sq: 50
    Magic { mask: 0x76080808080800, factor: 0x8208002004020020, offset: 24584 }, // sq: 51
    Magic { mask: 0x6e101010101000, factor: 0x9044010020020020, offset: 25608 }, // sq: 52
    Magic { mask: 0x5e202020202000, factor: 0x252008020010020, offset: 26632 }, // sq: 53
    Magic { mask: 0x3e404040404000, factor: 0x4054400100008040, offset: 27656 }, // sq: 54
    Magic { mask: 0x7e808080808000, factor: 0x6a6009400c7ae00, offset: 28680 }, // sq: 55
    Magic { mask: 0x17e0101010100, factor: 0xc1084009e08c8004, offset: 10241 }, // sq: 40
    Magic { mask: 0x27c0202020200, factor: 0x800200080408010, offset: 12289 }, // sq: 41
    Magic { mask: 0x47a0404040400, factor: 0x2000080010004040, offset: 13313 }, // sq: 42
    Magic { mask: 0x8760808080800, factor: 0x804004028004004, offset: 14337 }, // sq: 43
    Magic { mask: 0x106e1010101000, factor: 0x2004004004002, offset: 15369 }, // sq: 44
    Magic { mask: 0x205e2020202000, factor: 0x8b80010002004040, offset: 16393 }, // sq: 45
    Magic { mask: 0x403e4040404000, factor: 0x2000008001004040, offset: 17417 }, // sq: 46
    Magic { mask: 0x807e8080808000, factor: 0x81184c09402001f, offset: 18441 }, // sq: 47
    Magic { mask: 0x1017e01010100, factor: 0x348581400c800421, offset:     0 }, // sq: 32
    Magic { mask: 0x2027c02020200, factor: 0xd208001000200026, offset:  2048 }, // sq: 33
    Magic { mask: 0x4047a04040400, factor: 0x200400800401000, offset:  3072 }, // sq: 34
    Magic { mask: 0x8087608080800, factor: 0x4200400400400806, offset:  4096 }, // sq: 35
    Magic { mask: 0x10106e10101000, factor: 0xe60400400400200, offset:  5121 }, // sq: 36
    Magic { mask: 0x20205e20202000, factor: 0x1228400200400100, offset:  6145 }, // sq: 37
    Magic { mask: 0x40403e40404000, factor: 0x401000080202040, offset:  7169 }, // sq: 38
    Magic { mask: 0x80807e80808000, factor: 0x600041410200048c, offset:  8193 }, // sq: 39

    Magic { mask: 0x1017e01010100, factor: 0x348581400c800421, offset:     0 }, // sq: 32
    Magic { mask: 0x2027c02020200, factor: 0xd208001000200026, offset:  2048 }, // sq: 33
    Magic { mask: 0x4047a04040400, factor: 0x200400800401000, offset:  3072 }, // sq: 34
    Magic { mask: 0x8087608080800, factor: 0x4200400400400806, offset:  4096 }, // sq: 35
    Magic { mask: 0x10106e10101000, factor: 0xe60400400400200, offset:  5121 }, // sq: 36
    Magic { mask: 0x20205e20202000, factor: 0x1228400200400100, offset:  6145 }, // sq: 37
    Magic { mask: 0x40403e40404000, factor: 0x401000080202040, offset:  7169 }, // sq: 38
    Magic { mask: 0x80807e80808000, factor: 0x600041410200048c, offset:  8193 }, // sq: 39
    Magic { mask: 0x17e0101010100, factor: 0xc1084009e08c8004, offset: 10241 }, // sq: 40
    Magic { mask: 0x27c0202020200, factor: 0x800200080408010, offset: 12289 }, // sq: 41
    Magic { mask: 0x47a0404040400, factor: 0x2000080010004040, offset: 13313 }, // sq: 42
    Magic { mask: 0x8760808080800, factor: 0x804004028004004, offset: 14337 }, // sq: 43
    Magic { mask: 0x106e1010101000, factor: 0x2004004004002, offset: 15369 }, // sq: 44
    Magic { mask: 0x205e2020202000, factor: 0x8b80010002004040, offset: 16393 }, // sq: 45
    Magic { mask: 0x403e4040404000, factor: 0x2000008001004040, offset: 17417 }, // sq: 46
    Magic { mask: 0x807e8080808000, factor: 0x81184c09402001f, offset: 18441 }, // sq: 47
    Magic { mask: 0x7e010101010100, factor: 0xf0a5009e00b02a00, offset: 20489 }, // sq: 48
    Magic { mask: 0x7c020202020200, factor: 0x9aa0001004080010, offset: 22536 }, // sq: 49
    Magic { mask: 0x7a040404040400, factor: 0x8c8040200100010, offset: 23560 }, // sq: 50
    Magic { mask: 0x76080808080800, factor: 0x8208002004020020, offset: 24584 }, // sq: 51
    Magic { mask: 0x6e101010101000, factor: 0x9044010020020020, offset: 25608 }, // sq: 52
    Magic { mask: 0x5e202020202000, factor: 0x252008020010020, offset: 26632 }, // sq: 53
    Magic { mask: 0x3e404040404000, factor: 0x4054400100008040, offset: 27656 }, // sq: 54
    Magic { mask: 0x7e808080808000, factor: 0x6a6009400c7ae00, offset: 28680 }, // sq: 55
    Magic { mask: 0x7e01010101010100, factor: 0xebffffb9ff9fc526, offset: 30727 }, // sq: 56
    Magic { mask: 0x7c02020202020200, factor: 0x100112010e401486, offset: 32775 }, // sq: 57
    Magic { mask: 0x7a04040404040400, factor: 0xb0404a2000470901, offset: 34822 }, // sq: 58
    Magic { mask: 0x7608080808080800, factor: 0x10a00a0018400c86, offset: 36869 }, // sq: 59
    Magic { mask: 0x6e10101010101000, factor: 0x9110008003a1085, offset: 38915 }, // sq: 60
    Magic { mask: 0x5e20202020202000, factor: 0x5009208040005, offset: 40963 }, // sq: 61
    Magic { mask: 0x3e40404040404000, factor: 0x2d28087008a21094, offset: 43011 }, // sq: 62
    Magic { mask: 0x7e80808080808000, factor: 0x7645fffecbfea79e, offset: 45058 }, // sq: 63
]; // size: 47106

#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
pub static BISHOP_MAGICS: [Magic; 64] = [
    Magic { mask: 0x40201008040200, factor: 0x1584004200401002, offset:     0 }, // sq: 0
    Magic { mask: 0x402010080400, factor: 0x1040408080100200, offset:    64 }, // sq: 1
    Magic { mask: 0x4020100a00, factor: 0xe00802004216bc6, offset:    96 }, // sq: 2
    Magic { mask: 0x40221400, factor: 0x204080481140c580, offset:   128 }, // sq: 3
    Magic { mask: 0x2442800, factor: 0x54a0440212415152, offset:   176 }, // sq: 4
    Magic { mask: 0x204085000, factor: 0x21b021c100ae1012, offset:   239 }, // sq: 5
    Magic { mask: 0x20408102000, factor: 0x49102080440062a0, offset:   299 }, // sq: 6
    Magic { mask: 0x2040810204000, factor: 0x8848410040420040, offset:   361 }, // sq: 7
    Magic { mask: 0x20100804020000, factor: 0x8c08808021004011, offset:   487 }, // sq: 8
    Magic { mask: 0x40201008040000, factor: 0x8090420080100408, offset:   519 }, // sq: 9
    Magic { mask: 0x4020100a0000, factor: 0x10088100080810a8, offset:   551 }, // sq: 10
    Magic { mask: 0x4022140000, factor: 0x1010808048110102, offset:   583 }, // sq: 11
    Magic { mask: 0x244280000, factor: 0x80200044022150a8, offset:   631 }, // sq: 12
    Magic { mask: 0x20408500000, factor: 0x8ec70021c1009008, offset:   694 }, // sq: 13
    Magic { mask: 0x2040810200000, factor: 0xc0a2001041040048, offset:   754 }, // sq: 14
    Magic { mask: 0x4081020400000, factor: 0x5256681010820028, offset:   816 }, // sq: 15
    Magic { mask: 0x10080402000200, factor: 0xc000488202004010, offset:   878 }, // sq: 16
    Magic { mask: 0x20100804000400, factor: 0xc02048040404005, offset:   910 }, // sq: 17
    Magic { mask: 0x4020100a000a00, factor: 0x200800042004040, offset:   942 }, // sq: 18
    Magic { mask: 0x402214001400, factor: 0x500200200801081, offset:  1070 }, // sq: 19
    Magic { mask: 0x24428002800, factor: 0xc1a0240080841014, offset:  1198 }, // sq: 20
    Magic { mask: 0x2040850005000, factor: 0xa200080080840091, offset:  1336 }, // sq: 21
    Magic { mask: 0x4081020002000, factor: 0xac00083020210022, offset:  1464 }, // sq: 22
    Magic { mask: 0x8102040004000, factor: 0x4080040020104018, offset:  1496 }, // sq: 23
    Magic { mask: 0x8040200020400, factor: 0x8500804804010021, offset:  1528 }, // sq: 24
    Magic { mask: 0x10080400040800, factor: 0x8201002101002021, offset:  1560 }, // sq: 25
    Magic { mask: 0x20100a000a1000, factor: 0x4420820500420008, offset:  1592 }, // sq: 26
    Magic { mask: 0x40221400142200, factor: 0x106002088008020, offset:  1720 }, // sq: 27
    Magic { mask: 0x2442800284400, factor: 0x59080421004010, offset:  2232 }, // sq: 28
    Magic { mask: 0x4085000500800, factor: 0x2000108010404042, offset:  2744 }, // sq: 29
    Magic { mask: 0x8102000201000, factor: 0x55c0200640404020, offset:  2872 }, // sq: 30
    Magic { mask: 0x10204000402000, factor: 0x80000802c8104020, offset:  2904 }, // sq: 31
    Magic { mask: 0x4020002040800, factor: 0x2a20410402804040, offset:  2936 }, // sq: 32
    Magic { mask: 0x8040004081000, factor: 0xf030210021004080, offset:  2968 }, // sq: 33
    Magic { mask: 0x100a000a102000, factor: 0x40b0420040008102, offset:  3000 }, // sq: 34
    Magic { mask: 0x22140014224000, factor: 0xbe14420082080180, offset:  3128 }, // sq: 35
    Magic { mask: 0x44280028440200, factor: 0x30a842c0c0840100, offset:  3640 }, // sq: 36
    Magic { mask: 0x8500050080400, factor: 0x148002020904012, offset:  4152 }, // sq: 37
    Magic { mask: 0x10200020100800, factor: 0x120101011d04040, offset:  4280 }, // sq: 38
    Magic { mask: 0x20400040201000, factor: 0x3100200811202004, offset:  4312 }, // sq: 39
    Magic { mask: 0x2000204081000, factor: 0x210100890046, offset:  4344 }, // sq: 40
    Magic { mask: 0x4000408102000, factor: 0x1a0108080418022, offset:  4376 }, // sq: 41
    Magic { mask: 0xa000a10204000, factor: 0x800404200430080, offset:  4408 }, // sq: 42
    Magic { mask: 0x14001422400000, factor: 0x2049408840444020, offset:  4536 }, // sq: 43
    Magic { mask: 0x28002844020000, factor: 0x202a820040130100, offset:  4669 }, // sq: 44
    Magic { mask: 0x50005008040200, factor: 0xa610004100092040, offset:  4797 }, // sq: 45
    Magic { mask: 0x20002010080400, factor: 0x1010010048810, offset:  4925 }, // sq: 46
    Magic { mask: 0x40004020100800, factor: 0x8740200840092810, offset:  4957 }, // sq: 47
    Magic { mask: 0x20408102000, factor: 0x5410104202004232, offset:  4989 }, // sq: 48
    Magic { mask: 0x40810204000, factor: 0x408108021003025, offset:  5051 }, // sq: 49
    Magic { mask: 0xa1020400000, factor: 0x14081010806062, offset:  5113 }, // sq: 50
    Magic { mask: 0x142240000000, factor: 0x11228c1008403082, offset:  5149 }, // sq: 51
    Magic { mask: 0x284402000000, factor: 0x800420100202000, offset:  5210 }, // sq: 52
    Magic { mask: 0x500804020000, factor: 0x8a01008020802045, offset:  5242 }, // sq: 53
    Magic { mask: 0x201008040200, factor: 0x8901004100082002, offset:  5274 }, // sq: 54
    Magic { mask: 0x402010080400, factor: 0x84c040408004104d, offset:  5306 }, // sq: 55
    Magic { mask: 0x2040810204000, factor: 0x2104804040440054, offset:  5338 }, // sq: 56
    Magic { mask: 0x4081020400000, factor: 0x601002080404031, offset:  5464 }, // sq: 57
    Magic { mask: 0xa102040000000, factor: 0xa84ab18010108060, offset:  5526 }, // sq: 58
    Magic { mask: 0x14224000000000, factor: 0x3810200440084031, offset:  5562 }, // sq: 59
    Magic { mask: 0x28440200000000, factor: 0x2ca009201002020, offset:  5623 }, // sq: 60
    Magic { mask: 0x50080402000000, factor: 0x2c0070202002008, offset:  5655 }, // sq: 61
    Magic { mask: 0x20100804020000, factor: 0x5aa2810021004009, offset:  5687 }, // sq: 62
    Magic { mask: 0x40201008040200, factor: 0xc804040010401005, offset:  5719 }, // sq: 63
    // generation: 23 size: 5783 t: 8776ms
];
