fn main() {
    let input: Vec<Vec<&str>> = include_str!("./day_10.input").trim()
        .split("\n")
        .map(|x| x.split(" ").collect())
        .collect();

    let mut cycle: i32 = 0;
    let mut x: i32 = 1;

    for instruction in input {
        match instruction[0] {
            "noop" => {
                cycle += 1;
                if (x..=x+2).contains(&(cycle % 40)) {
                    print!("#");
                }
                else {
                    print!(".");
                }
                if cycle % 40 == 0 {
                    println!("");
                }
            },
            "addx" => {
                for _ in 0..2 {
                    cycle += 1;
                    if (x..=x+2).contains(&(cycle % 40)) {
                        print!("#");
                    }
                    else {
                        print!(".");
                    }
                    if cycle % 40 == 0 {
                        println!("");
                    }
                }
                x += instruction[1].parse::<i32>().unwrap();
            },
            _ => println!("Invalid instruction")
        }
    }
}
