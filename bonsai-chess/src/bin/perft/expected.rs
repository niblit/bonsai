use crate::perft_results::PerftResults; // Adjust path as needed

pub const PERFT_EXPECTED: [PerftResults; 9] = [
    // Depth 0
    PerftResults {
        nodes: 1,
        captures: 0,
        en_passant: 0,
        castles: 0,
        promotions: 0,
    },
    // Depth 1
    PerftResults {
        nodes: 20,
        captures: 0,
        en_passant: 0,
        castles: 0,
        promotions: 0,
    },
    // Depth 2
    PerftResults {
        nodes: 400,
        captures: 0,
        en_passant: 0,
        castles: 0,
        promotions: 0,
    },
    // Depth 3
    PerftResults {
        nodes: 8_902,
        captures: 34,
        en_passant: 0,
        castles: 0,
        promotions: 0,
    },
    // Depth 4
    PerftResults {
        nodes: 197_281,
        captures: 1_576,
        en_passant: 0,
        castles: 0,
        promotions: 0,
    },
    // Depth 5
    PerftResults {
        nodes: 4_865_609,
        captures: 82_719,
        en_passant: 258,
        castles: 0,
        promotions: 0,
    },
    // Depth 6
    PerftResults {
        nodes: 119_060_324,
        captures: 2_812_008,
        en_passant: 5_248,
        castles: 0,
        promotions: 0,
    },
    // Depth 7
    PerftResults {
        nodes: 3_195_901_860,
        captures: 108_329_926,
        en_passant: 319_617,
        castles: 883_453,
        promotions: 0,
    },
    // Depth 8
    PerftResults {
        nodes: 84_998_978_956,
        captures: 3_523_740_106,
        en_passant: 7_187_977,
        castles: 23_605_205,
        promotions: 0,
    },
];
