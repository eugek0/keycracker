use crate::{dialog::progress_bar, models::options::StartOptions};
use enigo::{Button, Direction, Enigo, Key, Keyboard, Mouse, Settings};
use rand::{Rng, rng};
use std::{
    collections::HashSet,
    io::{self, Write},
};

pub fn start(options: StartOptions) {
    let StartOptions {
        charset,
        length,
        total,
        method,
    } = options;

    match method {
        1 => random(charset, length, total),
        2 => increment(charset, length, total),
        _ => eprintln!("Unknown generation method"),
    }
}

fn gen_random(charset: &Vec<char>, length: usize) -> String {
    let mut random = rng();
    (0..length)
        .map(|_| charset[random.random_range(0..charset.len())])
        .collect()
}

pub fn random(charset: Vec<char>, length: u32, total: u32) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let mut used = HashSet::with_capacity(total as usize);

    while used.len() < total as usize {
        let string = gen_random(&charset, length as usize);
        if used.insert(string.clone()) {
            enigo.button(Button::Left, Direction::Click).unwrap();
            enigo.text(&string).unwrap();
            enigo.key(Key::Return, Direction::Press).unwrap();

            let count = used.len();
            let progress = count as f64 / total as f64;
            let bar = progress_bar(progress, 40);

            print!("\r{} / {}  {}", count.to_string(), total.to_string(), bar);
            io::stdout().flush().unwrap();
        }
    }
}

pub fn indices_to_string(charset: &[char], indices: &[usize]) -> String {
    indices.iter().map(|&i| charset[i]).collect()
}

pub fn increment_indices(indices: &mut [usize], base: usize) -> bool {
    for i in (0..indices.len()).rev() {
        if indices[i] + 1 < base {
            indices[i] += 1;
            return true;
        } else {
            indices[i] = 0;
        }
    }
    false
}

pub fn increment(charset: Vec<char>, length: u32, total: u32) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let mut indices = vec![0; length as usize];
    let total_possible_f64 = total as f64;
    let mut count = 0;

    loop {
        let string = indices_to_string(&charset, &indices);
        enigo.button(Button::Left, Direction::Click).unwrap();
        enigo.text(&string).unwrap();
        enigo.key(Key::Return, Direction::Press).unwrap();

        let progress = count as f64 / total_possible_f64;
        let bar = progress_bar(progress, 40);

        print!("\r{} / {}  {}", count.to_string(), total.to_string(), bar);
        io::stdout().flush().unwrap();

        count += 1;
        if !increment_indices(&mut indices, charset.len()) {
            break;
        }
    }
}
