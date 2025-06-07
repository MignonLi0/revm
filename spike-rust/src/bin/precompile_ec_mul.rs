use revm::{
    precompile::{
        bn128::{mul::ISTANBUL_MUL_GAS_COST, run_add, run_mul},
        secp256k1::{ec_recover_run, ecrecover},
    },
    primitives::{hex, Bytes},
};
use secp256k1::{
    ecdsa::{RecoverableSignature, RecoveryId},
    Message, Secp256k1,
};

fn main() {
    let ecmul_input = hex::decode(
        "\
         18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9\
         063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f37266\
         06614e20c147e940f2d70da3f74c9a17df361706a4485c742bd6788478fa17d7",
    )
    .unwrap();
    let input = Bytes::from(ecmul_input);

    let count = 2_0000;
    println!("Unit Count = {}", count);

    for _ in 0..count {
        let res = run_mul(&input, ISTANBUL_MUL_GAS_COST, u64::MAX).unwrap();
        std::hint::black_box(res);
    }
    println!("Gas/Unit = 6000");
}
