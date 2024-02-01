use rand::seq::SliceRandom;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Welcome!\nType something you'd like to be shuffled and sorted :)");
    let mut input_text = String::new();

    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");

    let input_text = input_text.trim();
    let text_vector: Vec<(char, u32)> = input_text
        .char_indices()
        .map(|(i, c)| (c, i as u32))
        .collect();
    let mut shuffled_vector = text_vector.clone();
    shuffled_vector.shuffle(&mut rand::thread_rng());
    //Choosing your sort

    println!("Type 1 to bubble sort, 2 to gnome sort");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess {
            1 => {
                bubble_sort(&mut shuffled_vector);
                break;
            }
            2 => {
                gnome_sort(&mut shuffled_vector);
                break;
            }
            _ => println!("Type a number between 1 and 2"),
        };
    }


    let characters: Vec<char> = shuffled_vector.iter().map(|&(c, _)| c).collect();
    let result_string: String = characters.into_iter().collect();
    println!("{}", result_string);
}
fn bubble_sort(array: &mut Vec<(char, u32)>) {
    let mut n = array.len();
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..n {
            if array[i - 1].1 > array[i].1 {
                array.swap(i - 1, i);
                let characters: Vec<char> = array.iter().map(|&(c, _)| c).collect();
                let result_string: String = characters.into_iter().collect();
                println!("{}", result_string);
                thread::sleep(Duration::from_millis(200));
                print!("\x1B[2J");
                swapped = true;
            }
        }
        n -= 1;
    }
}
fn gnome_sort(array: &mut Vec<(char, u32)>) {
    let mut pos = 0;
    while pos < array.len() {
        if pos == 0 || array[pos - 1].1 <= array[pos].1 {
            // Move to the next position if current element is in order
            pos += 1;
        } else {
            // Swap current element with the previous one and move one step backward
            array.swap(pos, pos - 1);
            pos = pos.saturating_sub(1); // Ensure pos never goes below 0
                let characters: Vec<char> = array.iter().map(|&(c, _)| c).collect();
                let result_string: String = characters.into_iter().collect();
                println!("{}", result_string);
                thread::sleep(Duration::from_millis(200));
                print!("\x1B[2J");
        }
    }
}
