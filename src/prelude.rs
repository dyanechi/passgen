// use clap::Parser;
use rand::rngs::ThreadRng;

pub use super::ui;

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// pub struct Args {
//     #[arg(short, long, default_value_t = 16)]
//     pub length: usize,
// }

// pub struct Config {
//     pub args: Args,
//     pub rng: ThreadRng,
// }

// pub fn init() -> Config {
//     Config {
//         args: Args::parse(),
//         rng: rand::thread_rng(),
//     }
// }