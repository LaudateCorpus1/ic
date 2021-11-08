use rand_core::RngCore;
use tecdsa::*;

#[test]
fn seed_fixed_output() {
    fn test_seed_output(seed: Seed, expected: &str) {
        let mut rng = seed.into_rng();

        let mut rng_output = vec![0u8; expected.len() / 2];
        rng.fill_bytes(&mut rng_output);
        assert_eq!(hex::encode(rng_output), expected);
    }

    /*
    These test outputs were generated by our own implementation of Seed, without
    any crosschecking. The main purpose is to guarantee that the behavior of
    Seed cannot silently change over time, for example due to a change in a
    dependency such as rand_chacha.
    */

    test_seed_output(
        Seed::from_bytes(&[42; 32]),
        "37cca0b1183b7c084731a1e0445707aa5f52119e69dd582bfc3c6b9a79225523",
    );

    test_seed_output(
        Seed::from_bytes(&[42; 32]).derive("label1"),
        "c10cc8472945b48a8794da965a0e1110c8178943f2fbecc4bca52c95d0b8a5c3",
    );

    test_seed_output(
        Seed::from_bytes(&[42; 32])
            .derive("label1")
            .derive("label2"),
        "4119033db2534e972e328209f61bf69f722da4e7c5a277799c3d5ddfefa6be7a",
    );

    let mut rng = Seed::from_bytes(&[42; 32]).into_rng();

    test_seed_output(
        Seed::from_rng(&mut rng),
        "d73d27ddc5e54cfd69941f005b0ce62997b88a212f3513c5eb56ea5532a31ccf",
    );

    // This reads the next bytes from the RNG and so has a different
    // output than the previous test
    test_seed_output(
        Seed::from_rng(&mut rng),
        "3f811c646ec91b746ac4a41fdcc86f5a95911297e2c1006cb77f0e58013170b8",
    );
}