use revm::{
    precompile::{
        blake2::run,
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
    let mut input = [1; 213];

    input[0] = 0;
    input[1] = 0;
    input[3] = 0;
    //round = 256

    let count = 5000;
    println!("Unit Count = {}", count);

    for _ in 0..count {
        let res = run(&Bytes::from(input), 10000000000).unwrap();
        std::hint::black_box(res);
    }

    println!("Gas/Unit = 256");
}
