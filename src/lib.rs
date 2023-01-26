pub mod prelude;
pub mod rgen;
pub mod ui;

pub use prelude::*;




pub fn welcome() {
    let welcome_msg = "--- Welcome to PSG - Password Generator! ---";
    println!("{}", "=".repeat(welcome_msg.len()));
    println!("{}", welcome_msg);
    println!("{}", "=".repeat(welcome_msg.len()));
}