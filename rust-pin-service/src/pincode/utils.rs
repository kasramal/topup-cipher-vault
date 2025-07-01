use rand::{thread_rng, Rng};
use rand::distributions::Uniform;

pub fn generate_random_pin(length: usize) -> String {
    let mut rng = thread_rng();
    let digits = Uniform::from(0..10);
    (0..length).map(|_| rng.sample(&digits).to_string()).collect()
} 