mod dialog;
mod exit;
mod generators;
mod models;

use device_query::{DeviceQuery, DeviceState, Keycode};
use dialog::{chars, number};
use exit::watch_exit;
use generators::start;
use models::options::StartOptions;

fn main() {
    watch_exit();

    let state = DeviceState::new();

    let charset = chars("Enter the characters that will make up the password: ");
    let length = number("Enter password length: ");

    let total = charset.len().pow(length) as u32;

    println!(
        "\nYou entered: {}.\nPassword length: {}.\nNumber of possible options: {}.\n",
        charset.iter().collect::<String>(),
        length,
        total
    );

    let method = number(
        "(1) Generate random strings.\n(2) Lexicographic search.\nSelect a password generation method: ",
    ) as u8;

    println!("To start, press F6...");

    // Block until F6 is not pressed
    loop {
        if state.get_keys().contains(&Keycode::F6) {
            break;
        };
    }

    start(StartOptions {
        charset,
        length,
        total,
        method,
    });
}
