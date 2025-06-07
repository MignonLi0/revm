use revm::{
    precompile::secp256k1::{ec_recover_run, ecrecover},
    primitives::Bytes,
};
use secp256k1::{
    ecdsa::{RecoverableSignature, RecoveryId},
    Message, Secp256k1,
};

fn main() {
    let input = [
        212, 149, 38, 245, 233, 106, 212, 120, 107, 179, 69, 13, 27, 124, 113, 32, 191, 39, 211,
        160, 63, 224, 78, 199, 11, 33, 114, 19, 124, 240, 163, 126, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 20, 67, 42, 2, 159, 244,
        238, 66, 166, 28, 86, 95, 246, 219, 136, 129, 229, 57, 80, 214, 78, 250, 23, 2, 36, 19, 76,
        1, 240, 9, 26, 134, 55, 118, 35, 140, 22, 70, 56, 104, 106, 138, 130, 95, 32, 18, 8, 90,
        245, 88, 100, 217, 234, 249, 75, 141, 204, 144, 69, 75, 56, 247, 207, 228,
    ];

    let count = 50000;
    println!("Unit Count = {}", count);

    for _ in 0..count {
        let res = ec_recover_run(&Bytes::from(input), u64::MAX).unwrap();
        std::hint::black_box(res);
    }

    println!("Gas/Unit = 3000");
}
