use crypto_hashes::digest::Digest;
use revm::{
    precompile::hash::{ripemd160_run, sha256_run},
    primitives::Bytes,
};

fn main() {
    const SIZE: usize = 10 * 48 * 1024;
    let input = [0; SIZE];

    println!("Unit Count = {}", 49152000);

    for _ in 0..100 {
        let res = sha256_run(&Bytes::from(input), 100_000_000).unwrap();
        std::hint::black_box(res);
    }

    println!("Gas/Unit = 0.375");
}
