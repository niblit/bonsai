use crate::perft_results::PerftResults;

/// The known correct Perft results for the standard starting position.
///
/// These values verify that the move generator is working correctly by comparing
/// the engine's output against established chess engine standards.
///
/// Source: [CPW - Perft Results](https://www.chessprogramming.org/Perft_Results)
/// You normally only test up to depth ~6, because larger depths take a lot of
/// time, but added up to depth 8 just for completness
pub const PERFT_EXPECTED: [PerftResults; 9] = [
    // Depth 0: 1 node (the starting position itself)
    PerftResults {
        nodes: 1,
        captures: 0,
        en_passant: 0,
        castles: 0,
        promotions: 0,
    },
    // Depth 1: 20 legal moves
    PerftResults {
        nodes: 20,
        captures: 0,
        en_passant: 0,
        castles: 0,
        promotions: 0,
    },
    // Depth 2: 400 leaf nodes
    PerftResults {
        nodes: 400,
        captures: 0,
        en_passant: 0,
        castles: 0,
        promotions: 0,
    },
    // Depth 3: 8,902 leaf nodes
    PerftResults {
        nodes: 8_902,
        captures: 34,
        en_passant: 0,
        castles: 0,
        promotions: 0,
    },
    // Depth 4: 197,281 leaf nodes
    PerftResults {
        nodes: 197_281,
        captures: 1_576,
        en_passant: 0,
        castles: 0,
        promotions: 0,
    },
    // Depth 5: 4,865,609 leaf nodes
    PerftResults {
        nodes: 4_865_609,
        captures: 82_719,
        en_passant: 258,
        castles: 0,
        promotions: 0,
    },
    // Depth 6: 119,060,324 leaf nodes
    PerftResults {
        nodes: 119_060_324,
        captures: 2_812_008,
        en_passant: 5_248,
        castles: 0,
        promotions: 0,
    },
    // Depth 7: 3,195,901,860 leaf nodes
    PerftResults {
        nodes: 3_195_901_860,
        captures: 108_329_926,
        en_passant: 319_617,
        castles: 883_453,
        promotions: 0,
    },
    // Depth 8: 84,998,978,956 leaf nodes
    PerftResults {
        nodes: 84_998_978_956,
        captures: 3_523_740_106,
        en_passant: 7_187_977,
        castles: 23_605_205,
        promotions: 0,
    },
];
