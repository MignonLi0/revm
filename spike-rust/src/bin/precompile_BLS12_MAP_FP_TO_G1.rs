use revm::{
    precompile::bls12_381::map_fp_to_g1::PRECOMPILE,
    primitives::{hex, Bytes},
};

fn main() {
    let input = hex::decode(
        "0x0000000000000000000000000000000016f6b59f8df4344269685680b9e2e3750321051ca0f8e064d480e2a57f07073e609993e1667326b477ddb78ac52b3e8a",
    )
    .unwrap();
    let input_bytes = Bytes::copy_from_slice(&input);

    let count = 5000;
    println!("Unit Count = {}", count);

    for _ in 0..count {
        let precompile = *PRECOMPILE.precompile();
        let res = precompile(&input_bytes, u64::MAX).unwrap();
        std::hint::black_box(res);
    }

    println!("Gas/Unit = 5500");
}
