use crate::board::Bitboard;

pub const DEFAULT_FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const BB_A_FILE: Bitboard = 0x0101010101010101;
pub const BB_B_FILE: Bitboard = BB_A_FILE << 1;
pub const BB_C_FILE: Bitboard = BB_A_FILE << 2;
pub const BB_D_FILE: Bitboard = BB_A_FILE << 3;
pub const BB_E_FILE: Bitboard = BB_A_FILE << 4;
pub const BB_F_FILE: Bitboard = BB_A_FILE << 5;
pub const BB_G_FILE: Bitboard = BB_A_FILE << 6;
pub const BB_H_FILE: Bitboard = BB_A_FILE << 7;

pub const BB_1_RANK: Bitboard = 0xFF;
pub const BB_2_RANK: Bitboard = BB_1_RANK << (8 * 1);
pub const BB_3_RANK: Bitboard = BB_1_RANK << (8 * 2);
pub const BB_4_RANK: Bitboard = BB_1_RANK << (8 * 3);
pub const BB_5_RANK: Bitboard = BB_1_RANK << (8 * 4);
pub const BB_6_RANK: Bitboard = BB_1_RANK << (8 * 5);
pub const BB_7_RANK: Bitboard = BB_1_RANK << (8 * 6);
pub const BB_8_RANK: Bitboard = BB_1_RANK << (8 * 7);

pub const BOARD_SQUARES: [u64; 64] = [
    1,
    2,
    4,
    8,
    16,
    32,
    64,
    128,
    256,
    512,
    1024,
    2048,
    4096,
    8192,
    16384,
    32768,
    65536,
    131072,
    262144,
    524288,
    1048576,
    2097152,
    4194304,
    8388608,
    16777216,
    33554432,
    67108864,
    134217728,
    268435456,
    536870912,
    1073741824,
    2147483648,
    4294967296,
    8589934592,
    17179869184,
    34359738368,
    68719476736,
    137438953472,
    274877906944,
    549755813888,
    1099511627776,
    2199023255552,
    4398046511104,
    8796093022208,
    17592186044416,
    35184372088832,
    70368744177664,
    140737488355328,
    281474976710656,
    562949953421312,
    1125899906842624,
    2251799813685248,
    4503599627370496,
    9007199254740992,
    18014398509481984,
    36028797018963968,
    72057594037927936,
    144115188075855872,
    288230376151711744,
    576460752303423488,
    1152921504606846976,
    2305843009213693952,
    4611686018427387904,
    9223372036854775808,
];

pub const PAWN_ATTACK_SQUARES: [[u64; 64]; 2] = [
    [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        2,
        5,
        10,
        20,
        40,
        80,
        160,
        64,
        512,
        1280,
        2560,
        5120,
        10240,
        20480,
        40960,
        16384,
        131072,
        327680,
        655360,
        1310720,
        2621440,
        5242880,
        10485760,
        4194304,
        33554432,
        83886080,
        167772160,
        335544320,
        671088640,
        1342177280,
        2684354560,
        1073741824,
        8589934592,
        21474836480,
        42949672960,
        85899345920,
        171798691840,
        343597383680,
        687194767360,
        274877906944,
        2199023255552,
        5497558138880,
        10995116277760,
        21990232555520,
        43980465111040,
        87960930222080,
        175921860444160,
        70368744177664,
        562949953421312,
        1407374883553280,
        2814749767106560,
        5629499534213120,
        11258999068426240,
        22517998136852480,
        45035996273704960,
        18014398509481984,
    ],
    [
        512,
        1280,
        2560,
        5120,
        10240,
        20480,
        40960,
        16384,
        131072,
        327680,
        655360,
        1310720,
        2621440,
        5242880,
        10485760,
        4194304,
        33554432,
        83886080,
        167772160,
        335544320,
        671088640,
        1342177280,
        2684354560,
        1073741824,
        8589934592,
        21474836480,
        42949672960,
        85899345920,
        171798691840,
        343597383680,
        687194767360,
        274877906944,
        2199023255552,
        5497558138880,
        10995116277760,
        21990232555520,
        43980465111040,
        87960930222080,
        175921860444160,
        70368744177664,
        562949953421312,
        1407374883553280,
        2814749767106560,
        5629499534213120,
        11258999068426240,
        22517998136852480,
        45035996273704960,
        18014398509481984,
        144115188075855872,
        360287970189639680,
        720575940379279360,
        1441151880758558720,
        2882303761517117440,
        5764607523034234880,
        11529215046068469760,
        4611686018427387904,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
];

pub const KING_MOVES: [Bitboard; 64] = [
    770,
    1797,
    3594,
    7188,
    14376,
    28752,
    57504,
    49216,
    197123,
    460039,
    920078,
    1840156,
    3680312,
    7360624,
    14721248,
    12599488,
    50463488,
    117769984,
    235539968,
    471079936,
    942159872,
    1884319744,
    3768639488,
    3225468928,
    12918652928,
    30149115904,
    60298231808,
    120596463616,
    241192927232,
    482385854464,
    964771708928,
    825720045568,
    3307175149568,
    7718173671424,
    15436347342848,
    30872694685696,
    61745389371392,
    123490778742784,
    246981557485568,
    211384331665408,
    846636838289408,
    1975852459884544,
    3951704919769088,
    7903409839538176,
    15806819679076352,
    31613639358152704,
    63227278716305408,
    54114388906344448,
    216739030602088448,
    505818229730443264,
    1011636459460886528,
    2023272918921773056,
    4046545837843546112,
    8093091675687092224,
    16186183351374184448,
    13853283560024178688,
    144959613005987840,
    362258295026614272,
    724516590053228544,
    1449033180106457088,
    2898066360212914176,
    5796132720425828352,
    11592265440851656704,
    4665729213955833856,
];

pub const KNIGHT_MOVES: [Bitboard; 64] = [
    132096,
    329728,
    659712,
    1319424,
    2638848,
    5277696,
    10489856,
    4202496,
    33816580,
    84410376,
    168886289,
    337772578,
    675545156,
    1351090312,
    2685403152,
    1075839008,
    8657044482,
    21609056261,
    43234889994,
    86469779988,
    172939559976,
    345879119952,
    687463207072,
    275414786112,
    2216203387392,
    5531918402816,
    11068131838464,
    22136263676928,
    44272527353856,
    88545054707712,
    175990581010432,
    70506185244672,
    567348067172352,
    1416171111120896,
    2833441750646784,
    5666883501293568,
    11333767002587136,
    22667534005174272,
    45053588738670592,
    18049583422636032,
    145241105196122112,
    362539804446949376,
    725361088165576704,
    1450722176331153408,
    2901444352662306816,
    5802888705324613632,
    11533718717099671552,
    4620693356194824192,
    288234782788157440,
    576469569871282176,
    1224997833292120064,
    2449995666584240128,
    4899991333168480256,
    9799982666336960512,
    1152939783987658752,
    2305878468463689728,
    1128098930098176,
    2257297371824128,
    4796069720358912,
    9592139440717824,
    19184278881435648,
    38368557762871296,
    4679521487814656,
    9077567998918656,
];

pub const BB_ROOK_MOVES: [Bitboard; 64] = [
    282578800148862,
    565157600297598,
    1130315200595070,
    2260630401190014,
    4521260802379902,
    9042521604759678,
    18085043209519230,
    36170086419038334,
    282578800180992,
    565157600328704,
    1130315200625152,
    2260630401218048,
    4521260802403840,
    9042521604775424,
    18085043209518592,
    36170086419070464,
    282578808406272,
    565157608292864,
    1130315208328192,
    2260630408398848,
    4521260808540160,
    9042521608822784,
    18085043209388032,
    36170086427295744,
    282580914077952,
    565159647117824,
    1130317180306432,
    2260632246683648,
    4521262379438080,
    9042522644946944,
    18085043175964672,
    36170088532967424,
    283119966028032,
    565681586307584,
    1130822006735872,
    2261102847592448,
    4521664529305600,
    9042787892731904,
    18085034619584512,
    36170627584917504,
    421117265248512,
    699298018886144,
    1260057572672512,
    2381576680245248,
    4624614895390720,
    9110691325681664,
    18082844186263552,
    36308624884137984,
    35748425865691392,
    34905104758997504,
    34344362452452352,
    33222877839362048,
    30979908613181440,
    26493970160820224,
    17522093256097792,
    71635933484580864,
    9079539427579068672,
    9079822006379217408,
    9080387163979514880,
    9081517479180109824,
    9083778109581299712,
    9088299370383679488,
    9097341891988439040,
    9115426935197958144,
];
