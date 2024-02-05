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

    print!("\x1B[2J");
    println!("Type 1 to gnome sort, 2 to bubble sort, 3 to merge sort, 4 to radix sort");
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
                gnome_sort(&mut shuffled_vector);
                break;
            }
            2 => {
                bubble_sort(&mut shuffled_vector);
                break;
            }
            3 => {
                sort_tuples(&mut shuffled_vector);
                break;
            }
            4 => {
                radix_sort(&mut shuffled_vector);
                break;
            }
            _ => println!("Type a number between 1 and 4"),
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

                //Print text, wait, clear screen
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

// Radix sort function for a vector of tuples
fn radix_sort(array: &mut Vec<(char, u32)>) {
    let max = find_max(array);

    let mut exp = 1;
    while max / exp > 0 {
        counting_sort(array, exp);
        exp *= 10;
    }

    // Function to find the maximum value in the vector of tuples
    fn find_max(arr: &[(char, u32)]) -> u32 {
        arr.iter().map(|&(_, num)| num).max().unwrap_or(0)
    }

    // Function to perform counting sort based on a specific digit (place)
    fn counting_sort(arr: &mut [(char, u32)], exp: u32) {
        let mut output = vec![(0 as char, 0); arr.len()]; // Initialize output with tuple of appropriate types
        let mut count = vec![0; 10];
        //Print text, wait, clear screen
        let characters: Vec<char> = arr.iter().map(|&(c, _)| c).collect();
        let result_string: String = characters.into_iter().collect();
        println!("{}", result_string);
        thread::sleep(Duration::from_millis(2000));
        print!("\x1B[2J");

        for &(ch, num) in arr.iter() {
            // Use pattern matching to destructure the tuple
            let idx = (num / exp) % 10;
            count[idx as usize] += 1;
        }

        for i in 1..10 {
            count[i] += count[i - 1];
        }

        for &(ch, num) in arr.iter().rev() {
            // Use pattern matching to destructure the tuple
            let idx = (num / exp) % 10;
            output[count[idx as usize] as usize - 1] = (ch, num); // Assign tuple to output
            count[idx as usize] -= 1;
        }

        arr.copy_from_slice(&output);
    }
}

// Entry point for merge sort
fn sort_tuples(array: &mut Vec<(char, u32)>) {
    let n = array.len();
    merge_sort(array, 0, n - 1);

    // Function to merge two sorted subarrays
    fn merge(arr: &mut Vec<(char, u32)>, left: usize, mid: usize, right: usize) {
        let mut temp = Vec::with_capacity(right - left + 1);
        let (mut i, mut j) = (left, mid + 1);

        while i <= mid && j <= right {
            if arr[i].1 <= arr[j].1 {
                temp.push(arr[i]);
                i += 1;
            } else {
                temp.push(arr[j]);
                j += 1;
            }
        }

        while i <= mid {
            temp.push(arr[i]);
            i += 1;
        }

        while j <= right {
            temp.push(arr[j]);
            j += 1;
        }

        for (idx, tuple) in temp.iter().enumerate() {
            arr[left + idx] = *tuple;
        }
        //Print text, wait, clear screen
        let characters: Vec<char> = arr.iter().map(|&(c, _)| c).collect();
        let result_string: String = characters.into_iter().collect();
        println!("{}", result_string);
        thread::sleep(Duration::from_millis(200));
        print!("\x1B[2J");
    }

    // Merge sort function
    fn merge_sort(arr: &mut Vec<(char, u32)>, left: usize, right: usize) {
        if left < right {
            let mid = left + (right - left) / 2;
            merge_sort(arr, left, mid);
            merge_sort(arr, mid + 1, right);
            merge(arr, left, mid, right);
            //Print text, wait, clear screen
            let characters: Vec<char> = arr.iter().map(|&(c, _)| c).collect();
            let result_string: String = characters.into_iter().collect();
            println!("{}", result_string);
            thread::sleep(Duration::from_millis(200));
            print!("\x1B[2J");
        }
    }
}
