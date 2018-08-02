// vertical vector rotations can be better implemented with vector
// shuffles when the rotate distance is a multiple of 8

macro_rules! shuffle_bytes {
    ($x:expr, $vec8:ident, $vec:ident, $indices:tt) => {{
        let bytes: $vec8 = shuffle!($vec8::from_bits($x), $indices);
        $vec::from_bits(bytes)
    }};
}

#[macro_export]
macro_rules! rotate_left {

    /* +++++++++++ 32-bit lane width +++++++++++ */

    ($x:expr,8,u32x2) => {{
        shuffle_bytes!($x, u8x8, u32x2, [1,2,3,0,5,6,7,4])
    }};
    ($x:expr,8,u32x4) => {{
        shuffle_bytes!($x, u8x16, u32x4, [1,2,3,0,5,6,7,4,9,10,11,8,13,14,15,12])
    }};
    ($x:expr,8,u32x8) => {{
        shuffle_bytes!($x, u8x32, u32x8, [1,2,3,0,5,6,7,4,9,10,11,8,13,14,15,12,17,18,19,16,21,22,23,20,25,26,27,24,29,30,31,28])
    }};
    ($x:expr,8,u32x16) => {{
        shuffle_bytes!($x, u8x64, u32x16, [1,2,3,0,5,6,7,4,9,10,11,8,13,14,15,12,17,18,19,16,21,22,23,20,25,26,27,24,29,30,31,28,33,34,35,32,37,38,39,36,41,42,43,40,45,46,47,44,49,50,51,48,53,54,55,52,57,58,59,56,61,62,63,60])
    }};

    ($x:expr,16,u32x2) => {{
        shuffle_bytes!($x, u8x8, u32x2, [2,3,0,1,6,7,4,5])
    }};
    ($x:expr,16,u32x4) => {{
        shuffle_bytes!($x, u8x16, u32x4, [2,3,0,1,6,7,4,5,10,11,8,9,14,15,12,13])
    }};
    ($x:expr,16,u32x8) => {{
        shuffle_bytes!($x, u8x32, u32x8, [2,3,0,1,6,7,4,5,10,11,8,9,14,15,12,13,18,19,16,17,22,23,20,21,26,27,24,25,30,31,28,29])
    }};
    ($x:expr,16,u32x16) => {{
        shuffle_bytes!($x, u8x64, u32x16, [2,3,0,1,6,7,4,5,10,11,8,9,14,15,12,13,18,19,16,17,22,23,20,21,26,27,24,25,30,31,28,29,34,35,32,33,38,39,36,37,42,43,40,41,46,47,44,45,50,51,48,49,54,55,52,53,58,59,56,57,62,63,60,61])
    }};

    ($x:expr,24,u32x2) => {{
        shuffle_bytes!($x, u8x8, u32x2, [3,0,1,2,7,4,5,6])
    }};
    ($x:expr,24,u32x4) => {{
        shuffle_bytes!($x, u8x16, u32x4, [3,0,1,2,7,4,5,6,11,8,9,10,15,12,13,14])
    }};
    ($x:expr,24,u32x8) => {{
        shuffle_bytes!($x, u8x32, u32x8, [3,0,1,2,7,4,5,6,11,8,9,10,15,12,13,14,19,16,17,18,23,20,21,22,27,24,25,26,31,28,29,30])
    }};
    ($x:expr,24,u32x16) => {{
        shuffle_bytes!($x, u8x64, u32x16, [3,0,1,2,7,4,5,6,11,8,9,10,15,12,13,14,19,16,17,18,23,20,21,22,27,24,25,26,31,28,29,30,35,32,33,34,39,36,37,38,43,40,41,42,47,44,45,46,51,48,49,50,55,52,53,54,59,56,57,58,63,60,61,62])
    }};

    /* +++++++++++ 64-bit lane width +++++++++++ */

    ($x:expr,8,u64x2) => {{
        shuffle_bytes!($x, u8x16, u64x2, [1,2,3,4,5,6,7,0,9,10,11,12,13,14,15,8])
    }};
    ($x:expr,8,u64x4) => {{
        shuffle_bytes!($x, u8x32, u64x4, [1,2,3,4,5,6,7,0,9,10,11,12,13,14,15,8,17,18,19,20,21,22,23,16,25,26,27,28,29,30,31,24])
    }};
    ($x:expr,8,u64x8) => {{
        shuffle_bytes!($x, u8x64, u64x8, [1,2,3,4,5,6,7,0,9,10,11,12,13,14,15,8,17,18,19,20,21,22,23,16,25,26,27,28,29,30,31,24,33,34,35,36,37,38,39,32,41,42,43,44,45,46,47,40,49,50,51,52,53,54,55,48,57,58,59,60,61,62,63,56])
    }};

    ($x:expr,16,u64x2) => {{
        shuffle_bytes!($x, u8x16, u64x2, [2,3,4,5,6,7,0,1,10,11,12,13,14,15,8,9])
    }};
    ($x:expr,16,u64x4) => {{
        shuffle_bytes!($x, u8x32, u64x4, [2,3,4,5,6,7,0,1,10,11,12,13,14,15,8,9,18,19,20,21,22,23,16,17,26,27,28,29,30,31,24,25])
    }};
    ($x:expr,16,u64x8) => {{
        shuffle_bytes!($x, u8x64, u64x8, [2,3,4,5,6,7,0,1,10,11,12,13,14,15,8,9,18,19,20,21,22,23,16,17,26,27,28,29,30,31,24,25,34,35,36,37,38,39,32,33,42,43,44,45,46,47,40,41,50,51,52,53,54,55,48,49,58,59,60,61,62,63,56,57])
    }};

    ($x:expr,24,u64x2) => {{
        shuffle_bytes!($x, u8x16, u64x2, [3,4,5,6,7,0,1,2,11,12,13,14,15,8,9,10])
    }};
    ($x:expr,24,u64x4) => {{
        shuffle_bytes!($x, u8x32, u64x4, [3,4,5,6,7,0,1,2,11,12,13,14,15,8,9,10,19,20,21,22,23,16,17,18,27,28,29,30,31,24,25,26])
    }};
    ($x:expr,24,u64x8) => {{
        shuffle_bytes!($x, u8x64, u64x8, [3,4,5,6,7,0,1,2,11,12,13,14,15,8,9,10,19,20,21,22,23,16,17,18,27,28,29,30,31,24,25,26,35,36,37,38,39,32,33,34,43,44,45,46,47,40,41,42,51,52,53,54,55,48,49,50,59,60,61,62,63,56,57,58])
    }};

    ($x:expr,32,u64x2) => {{
        shuffle_bytes!($x, u8x16, u64x2, [4,5,6,7,0,1,2,3,12,13,14,15,8,9,10,11])
    }};
    ($x:expr,32,u64x4) => {{
        shuffle_bytes!($x, u8x32, u64x4, [4,5,6,7,0,1,2,3,12,13,14,15,8,9,10,11,20,21,22,23,16,17,18,19,28,29,30,31,24,25,26,27])
    }};
    ($x:expr,32,u64x8) => {{
        shuffle_bytes!($x, u8x64, u64x8, [4,5,6,7,0,1,2,3,12,13,14,15,8,9,10,11,20,21,22,23,16,17,18,19,28,29,30,31,24,25,26,27,36,37,38,39,32,33,34,35,44,45,46,47,40,41,42,43,52,53,54,55,48,49,50,51,60,61,62,63,56,57,58,59])
    }};

    ($x:expr,40,u64x2) => {{
        shuffle_bytes!($x, u8x16, u64x2, [5,6,7,0,1,2,3,4,13,14,15,8,9,10,11,12])
    }};
    ($x:expr,40,u64x4) => {{
        shuffle_bytes!($x, u8x32, u64x4, [5,6,7,0,1,2,3,4,13,14,15,8,9,10,11,12,21,22,23,16,17,18,19,20,29,30,31,24,25,26,27,28])
    }};
    ($x:expr,40,u64x8) => {{
        shuffle_bytes!($x, u8x64, u64x8, [5,6,7,0,1,2,3,4,13,14,15,8,9,10,11,12,21,22,23,16,17,18,19,20,29,30,31,24,25,26,27,28,37,38,39,32,33,34,35,36,45,46,47,40,41,42,43,44,53,54,55,48,49,50,51,52,61,62,63,56,57,58,59,60])
    }};

    ($x:expr,48,u64x2) => {{
        shuffle_bytes!($x, u8x16, u64x2, [6,7,0,1,2,3,4,5,14,15,8,9,10,11,12,13])
    }};
    ($x:expr,48,u64x4) => {{
        shuffle_bytes!($x, u8x32, u64x4, [6,7,0,1,2,3,4,5,14,15,8,9,10,11,12,13,22,23,16,17,18,19,20,21,30,31,24,25,26,27,28,29])
    }};
    ($x:expr,48,u64x8) => {{
        shuffle_bytes!($x, u8x64, u64x8, [6,7,0,1,2,3,4,5,14,15,8,9,10,11,12,13,22,23,16,17,18,19,20,21,30,31,24,25,26,27,28,29,38,39,32,33,34,35,36,37,46,47,40,41,42,43,44,45,54,55,48,49,50,51,52,53,62,63,56,57,58,59,60,61])
    }};

    ($x:expr,56,u64x2) => {{
        shuffle_bytes!($x, u8x16, u64x2, [7,0,1,2,3,4,5,6,15,8,9,10,11,12,13,14])
    }};
    ($x:expr,56,u64x4) => {{
        shuffle_bytes!($x, u8x32, u64x4, [7,0,1,2,3,4,5,6,15,8,9,10,11,12,13,14,23,16,17,18,19,20,21,22,31,24,25,26,27,28,29,30])
    }};
    ($x:expr,56,u64x8) => {{
        shuffle_bytes!($x, u8x64, u64x8, [7,0,1,2,3,4,5,6,15,8,9,10,11,12,13,14,23,16,17,18,19,20,21,22,31,24,25,26,27,28,29,30,39,32,33,34,35,36,37,38,47,40,41,42,43,44,45,46,55,48,49,50,51,52,53,54,63,56,57,58,59,60,61,62])
    }};

    // explicitly doing nothing for the zero case seems to compile better
    ($x:expr,0, $vector:ident) => {{
        $x
    }};

    // if the rotate distance is not divisible by 8, do a "shift + shift + or"
    ($x:expr, $rot:expr, $vector:ident) => {{
        $x.rotate_left($vector::splat($rot))
    }};
}

#[macro_export]
macro_rules! rotate_right {

    /* +++++++++++ 32-bit lane width +++++++++++ */

    ($x:expr,8,u32x2) => {{
        shuffle_bytes!($x, u8x8, u32x2, [3,0,1,2,7,4,5,6])
    }};
    ($x:expr,8,u32x4) => {{
        shuffle_bytes!($x, u8x16, u32x4, [3,0,1,2,7,4,5,6,11,8,9,10,15,12,13,14])
    }};
    ($x:expr,8,u32x8) => {{
        shuffle_bytes!($x, u8x32, u32x8, [3,0,1,2,7,4,5,6,11,8,9,10,15,12,13,14,19,16,17,18,23,20,21,22,27,24,25,26,31,28,29,30])
    }};
    ($x:expr,8,u32x16) => {{
        shuffle_bytes!($x, u8x64, u32x16, [3,0,1,2,7,4,5,6,11,8,9,10,15,12,13,14,19,16,17,18,23,20,21,22,27,24,25,26,31,28,29,30,35,32,33,34,39,36,37,38,43,40,41,42,47,44,45,46,51,48,49,50,55,52,53,54,59,56,57,58,63,60,61,62])
    }};

    ($x:expr,16,u32x2) => {{
        shuffle_bytes!($x, u8x8, u32x2, [2,3,0,1,6,7,4,5])
    }};
    ($x:expr,16,u32x4) => {{
        shuffle_bytes!($x, u8x16, u32x4, [2,3,0,1,6,7,4,5,10,11,8,9,14,15,12,13])
    }};
    ($x:expr,16,u32x8) => {{
        shuffle_bytes!($x, u8x32, u32x8, [2,3,0,1,6,7,4,5,10,11,8,9,14,15,12,13,18,19,16,17,22,23,20,21,26,27,24,25,30,31,28,29])
    }};
    ($x:expr,16,u32x16) => {{
        shuffle_bytes!($x, u8x64, u32x16, [2,3,0,1,6,7,4,5,10,11,8,9,14,15,12,13,18,19,16,17,22,23,20,21,26,27,24,25,30,31,28,29,34,35,32,33,38,39,36,37,42,43,40,41,46,47,44,45,50,51,48,49,54,55,52,53,58,59,56,57,62,63,60,61])
    }};

    ($x:expr,24,u32x2) => {{
        shuffle_bytes!($x, u8x8, u32x2, [1,2,3,0,5,6,7,4])
    }};
    ($x:expr,24,u32x4) => {{
        shuffle_bytes!($x, u8x16, u32x4, [1,2,3,0,5,6,7,4,9,10,11,8,13,14,15,12])
    }};
    ($x:expr,24,u32x8) => {{
        shuffle_bytes!($x, u8x32, u32x8, [1,2,3,0,5,6,7,4,9,10,11,8,13,14,15,12,17,18,19,16,21,22,23,20,25,26,27,24,29,30,31,28])
    }};
    ($x:expr,24,u32x16) => {{
        shuffle_bytes!($x, u8x64, u32x16, [1,2,3,0,5,6,7,4,9,10,11,8,13,14,15,12,17,18,19,16,21,22,23,20,25,26,27,24,29,30,31,28,33,34,35,32,37,38,39,36,41,42,43,40,45,46,47,44,49,50,51,48,53,54,55,52,57,58,59,56,61,62,63,60])
    }};

    /* +++++++++++ 64-bit lane width +++++++++++ */

    ($x:expr,8,u64x2) => {{
        shuffle_bytes!($x, u8x16, u64x2, [7,0,1,2,3,4,5,6,15,8,9,10,11,12,13,14])
    }};
    ($x:expr,8,u64x4) => {{
        shuffle_bytes!($x, u8x32, u64x4, [7,0,1,2,3,4,5,6,15,8,9,10,11,12,13,14,23,16,17,18,19,20,21,22,31,24,25,26,27,28,29,30])
    }};
    ($x:expr,8,u64x8) => {{
        shuffle_bytes!($x, u8x64, u64x8, [7,0,1,2,3,4,5,6,15,8,9,10,11,12,13,14,23,16,17,18,19,20,21,22,31,24,25,26,27,28,29,30,39,32,33,34,35,36,37,38,47,40,41,42,43,44,45,46,55,48,49,50,51,52,53,54,63,56,57,58,59,60,61,62])
    }};

    ($x:expr,16,u64x2) => {{
        shuffle_bytes!($x, u8x16, u64x2, [6,7,0,1,2,3,4,5,14,15,8,9,10,11,12,13])
    }};
    ($x:expr,16,u64x4) => {{
        shuffle_bytes!($x, u8x32, u64x4, [6,7,0,1,2,3,4,5,14,15,8,9,10,11,12,13,22,23,16,17,18,19,20,21,30,31,24,25,26,27,28,29])
    }};
    ($x:expr,16,u64x8) => {{
        shuffle_bytes!($x, u8x64, u64x8, [6,7,0,1,2,3,4,5,14,15,8,9,10,11,12,13,22,23,16,17,18,19,20,21,30,31,24,25,26,27,28,29,38,39,32,33,34,35,36,37,46,47,40,41,42,43,44,45,54,55,48,49,50,51,52,53,62,63,56,57,58,59,60,61])
    }};

    ($x:expr,24,u64x2) => {{
        shuffle_bytes!($x, u8x16, u64x2, [5,6,7,0,1,2,3,4,13,14,15,8,9,10,11,12])
    }};
    ($x:expr,24,u64x4) => {{
        shuffle_bytes!($x, u8x32, u64x4, [5,6,7,0,1,2,3,4,13,14,15,8,9,10,11,12,21,22,23,16,17,18,19,20,29,30,31,24,25,26,27,28])
    }};
    ($x:expr,24,u64x8) => {{
        shuffle_bytes!($x, u8x64, u64x8, [5,6,7,0,1,2,3,4,13,14,15,8,9,10,11,12,21,22,23,16,17,18,19,20,29,30,31,24,25,26,27,28,37,38,39,32,33,34,35,36,45,46,47,40,41,42,43,44,53,54,55,48,49,50,51,52,61,62,63,56,57,58,59,60])
    }};

    ($x:expr,32,u64x2) => {{
        shuffle_bytes!($x, u8x16, u64x2, [4,5,6,7,0,1,2,3,12,13,14,15,8,9,10,11])
    }};
    ($x:expr,32,u64x4) => {{
        shuffle_bytes!($x, u8x32, u64x4, [4,5,6,7,0,1,2,3,12,13,14,15,8,9,10,11,20,21,22,23,16,17,18,19,28,29,30,31,24,25,26,27])
    }};
    ($x:expr,32,u64x8) => {{
        shuffle_bytes!($x, u8x64, u64x8, [4,5,6,7,0,1,2,3,12,13,14,15,8,9,10,11,20,21,22,23,16,17,18,19,28,29,30,31,24,25,26,27,36,37,38,39,32,33,34,35,44,45,46,47,40,41,42,43,52,53,54,55,48,49,50,51,60,61,62,63,56,57,58,59])
    }};

    ($x:expr,40,u64x2) => {{
        shuffle_bytes!($x, u8x16, u64x2, [3,4,5,6,7,0,1,2,11,12,13,14,15,8,9,10])
    }};
    ($x:expr,40,u64x4) => {{
        shuffle_bytes!($x, u8x32, u64x4, [3,4,5,6,7,0,1,2,11,12,13,14,15,8,9,10,19,20,21,22,23,16,17,18,27,28,29,30,31,24,25,26])
    }};
    ($x:expr,40,u64x8) => {{
        shuffle_bytes!($x, u8x64, u64x8, [3,4,5,6,7,0,1,2,11,12,13,14,15,8,9,10,19,20,21,22,23,16,17,18,27,28,29,30,31,24,25,26,35,36,37,38,39,32,33,34,43,44,45,46,47,40,41,42,51,52,53,54,55,48,49,50,59,60,61,62,63,56,57,58])
    }};

    ($x:expr,48,u64x2) => {{
        shuffle_bytes!($x, u8x16, u64x2, [2,3,4,5,6,7,0,1,10,11,12,13,14,15,8,9])
    }};
    ($x:expr,48,u64x4) => {{
        shuffle_bytes!($x, u8x32, u64x4, [2,3,4,5,6,7,0,1,10,11,12,13,14,15,8,9,18,19,20,21,22,23,16,17,26,27,28,29,30,31,24,25])
    }};
    ($x:expr,48,u64x8) => {{
        shuffle_bytes!($x, u8x64, u64x8, [2,3,4,5,6,7,0,1,10,11,12,13,14,15,8,9,18,19,20,21,22,23,16,17,26,27,28,29,30,31,24,25,34,35,36,37,38,39,32,33,42,43,44,45,46,47,40,41,50,51,52,53,54,55,48,49,58,59,60,61,62,63,56,57])
    }};

    ($x:expr,56,u64x2) => {{
        shuffle_bytes!($x, u8x16, u64x2, [1,2,3,4,5,6,7,0,9,10,11,12,13,14,15,8])
    }};
    ($x:expr,56,u64x4) => {{
        shuffle_bytes!($x, u8x32, u64x4, [1,2,3,4,5,6,7,0,9,10,11,12,13,14,15,8,17,18,19,20,21,22,23,16,25,26,27,28,29,30,31,24])
    }};
    ($x:expr,56,u64x8) => {{
        shuffle_bytes!($x, u8x64, u64x8, [1,2,3,4,5,6,7,0,9,10,11,12,13,14,15,8,17,18,19,20,21,22,23,16,25,26,27,28,29,30,31,24,33,34,35,36,37,38,39,32,41,42,43,44,45,46,47,40,49,50,51,52,53,54,55,48,57,58,59,60,61,62,63,56])
    }};

    // explicitly doing nothing for the zero case seems to compile better
    ($x:expr,0, $vector:ident) => {{
        $x
    }};

    // if the rotate distance is not divisible by 8, do a "shift + shift + or"
    ($x:expr, $rot:expr, $vector:ident) => {{
        $x.rotate_right($vector::splat($rot))
    }};
}

// This generated the indices for the rotate macros
#[allow(dead_code)]
fn build_rotate_indices() {
    fn build_left(len: usize, block: usize, rot: usize) {
        let mut arr: Vec<_> = (0..len).collect();
        for idx in (0..len).step_by(block) {
            arr[idx..][..block].rotate_left(rot);
        }
        println!("{:?}", arr);
    }
    fn build_right(len: usize, block: usize, rot: usize) {
        let mut arr: Vec<_> = (0..len).collect();
        for idx in (0..len).step_by(block) {
            arr[idx..][..block].rotate_right(rot);
        }
        println!("{:?}", arr);
    }

    // 32-bit left
    for rot in 0..4 {
        build_left(8, 4, rot);
        build_left(16, 4, rot);
        build_left(32, 4, rot);
        build_left(64, 4, rot);
    }

    // 64-bit left
    for rot in 0..8 {
        build_left(16, 8, rot);
        build_left(32, 8, rot);
        build_left(64, 8, rot);
    }

    // 32-bit right
    for rot in 0..4 {
        build_right(8, 4, rot);
        build_right(16, 4, rot);
        build_right(32, 4, rot);
        build_right(64, 4, rot);
    }

    // 64-bit right
    for rot in 0..8 {
        build_right(16, 8, rot);
        build_right(32, 8, rot);
        build_right(64, 8, rot);
    }
}