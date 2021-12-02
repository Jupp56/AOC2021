use std::fs::*;

fn main() {
    let numbers: Vec<usize> = read_to_string("input.txt")
        .expect("Expected file input.txt!")
        .lines()
        .map(|x| {
            x.parse()
                .expect("non-number or non-positive number found in file!")
        })
        .collect();
    first_challenge(&numbers);
    second_challenge(&numbers);
}

fn first_challenge(numbers: &Vec<usize>) {
    let mut numbers = numbers.iter();
    let first = numbers.next();
    let first = match first {
        Some(first) => first,
        None => {
            println!("There was no number found!");
            return;
        }
    };
    let mut count = 0;
    let mut last_number = first;
    for number in numbers {
        if number > last_number {
            count += 1;
        }
        last_number = number;
    }
    println!("There were {} increases!", count);
}

fn second_challenge(numbers: &Vec<usize>) {
    let mut numbers = numbers.iter();
    let mut first_val = numbers
        .next()
        .expect("Not enough numbers for sliding average");
    let mut second_val = numbers
        .next()
        .expect("Not enough numbers for sliding average");
    let mut third_val = numbers
        .next()
        .expect("Not enough numbers for sliding average");
    let mut last_sum = None;
    let mut count = 0;
    loop {
        let sum = first_val + second_val + third_val;
        match last_sum {
            Some(last_sum) => {
                if sum > last_sum {
                    count += 1;
                }
            }
            None => {
                last_sum = Some(sum);
                continue;
            }
        }

        last_sum = Some(sum);

        let next = match numbers.next() {
            Some(next) => next,
            None => break,
        };
        first_val = second_val;
        second_val = third_val;
        third_val = next;
    }

    println!("Whith the rolling average, there were {} increases!", count);
}
