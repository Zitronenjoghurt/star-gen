use rand::Rng;

pub fn random_u64() -> u64 {
    let mut rng = rand::rng();
    rng.random::<u64>()
}
