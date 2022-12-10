fn main() {
    let input: Vec<Vec<&str>> = include_str!("./day_10.input").trim()
        .split("\n")
        .map(|x| x.split(" ").collect())
        .collect();

    let mut cycle: i32 = 0;
    let mut x: i32 = 1;
    let mut result: i32 = 0;

    for instruction in input {
        match instruction[0] {
            "noop" => cycle += 1,
            "addx" => {
                for _ in 0..2 {
                    cycle += 1;
                    if (cycle - 20) % 40 == 0 {
                        result += cycle * x;
                    }
                }
                x += instruction[1].parse::<i32>().unwrap();
            },
            _ => println!("Invalid instruction")
        }
    }
    println!("{result}");
}
