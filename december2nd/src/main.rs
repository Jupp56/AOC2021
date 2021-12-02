use std::fs::*;

struct Boat {
    horizontal: isize,
    depth: isize,
}

impl Default for Boat {
    fn default() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
        }
    }
}

fn main() {
    challenge1();
    challenge2();
}

fn challenge1() {
    let mut boat = Boat::default();
    read_to_string("input.txt")
        .expect("Expected file input.txt!")
        .lines()
        .for_each(|x| {
            let x = x.to_string();
            parse_and_run_instruction(x, &mut boat)
        });

    println!(
        "horizontal: {}, depth: {}, product: {}",
        boat.horizontal,
        boat.depth,
        boat.horizontal * boat.depth
    );
}

fn parse_and_run_instruction(line: String, boat: &mut Boat) {
    let movement: (isize, isize) = if line.contains("forward ") {
        let res = line.replace("forward ", "");

        (
            res.parse()
                .unwrap_or_else(|_| panic!("{} is no number!", res)),
            0,
        )
    } else if line.contains("up") {
        let res = line.replace("up ", "");
        (
            0,
            -res.parse::<isize>()
                .unwrap_or_else(|_| panic!("{} is no number!", res)),
        )
    } else if line.contains("down") {
        let res = line.replace("down ", "");
        (
            0,
            res.parse::<isize>()
                .unwrap_or_else(|_| panic!("{} is no number!", res)),
        )
    } else {
        panic!("Faulty input: {}", line);
    };

    boat.horizontal += movement.0;
    boat.depth += movement.1;
}

struct Boat2 {
    horizontal: isize,
    depth: isize,
    aim: isize,
}

impl Default for Boat2 {
    fn default() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

fn challenge2() {
    let mut boat = Boat2::default();
    read_to_string("input.txt")
        .expect("Expected file input.txt!")
        .lines()
        .for_each(|x| {
            let x = x.to_string();
            parse_and_run_instruction_2(x, &mut boat)
        });

    println!(
        "horizontal: {}, depth: {}, product: {}",
        boat.horizontal,
        boat.depth,
        boat.horizontal * boat.depth
    );
}

fn parse_and_run_instruction_2(line: String, boat: &mut Boat2) {
    if line.contains("forward ") {
        let res = line.replace("forward ", "");

        let x: isize = res
            .parse()
            .unwrap_or_else(|_| panic!("{} is no number!", res));

        boat.horizontal += x;
        boat.depth += boat.aim * x;
    } else if line.contains("up") {
        let res = line.replace("up ", "");
        boat.aim -= res
            .parse::<isize>()
            .unwrap_or_else(|_| panic!("{} is no number!", res));
    } else if line.contains("down") {
        let res = line.replace("down ", "");

        boat.aim += res
            .parse::<isize>()
            .unwrap_or_else(|_| panic!("{} is no number!", res));
    } else {
        panic!("Faulty input: {}", line);
    };
}
