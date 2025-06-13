use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{process, thread};

pub fn watch_exit() {
    thread::spawn(move || {
        let state = DeviceState::new();

        loop {
            if state.get_keys().contains(&Keycode::Escape) {
                process::exit(1);
            }
        }
    });
}
