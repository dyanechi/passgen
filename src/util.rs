use super::*;
use rand::{Rng, distributions::Alphanumeric};

pub fn alphanum(len: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
} 

// pub fn str(config: Config) -> String {
//     let (args, mut rng) = (config.args, config.rng);
//     let r_str: String = (0..args.length)
//         .map(|_| rng.sample(Alphanumeric) as char)
//         .collect();

//     println!("{}", r_str);
//     r_str
// }