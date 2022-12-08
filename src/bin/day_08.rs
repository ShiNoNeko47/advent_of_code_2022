fn main() {
    let input: Vec<Vec<u8>> = include_str!("./day_08.input").trim()
        .split("\n")
        .map(|x| x.chars()
             .map(|y| y.to_string().parse().unwrap())
             .collect())
        .collect();

    let x: usize = input[0].len();
    let y: usize = input.len();

    let mut result: usize = x * y;

    for i in 1..y-1 {
        for j in 1..x-1 {
            let height: u8 = input[i][j];
            let vertical: &Vec<u8> = &input.iter().map(|x| x[j]).collect();
            let horizontal: &Vec<u8> = &input[i];

            let max_heights: [&u8; 4] = [
                    vertical[0..i].iter().max().unwrap(),
                    vertical[i+1..].iter().max().unwrap(),
                    horizontal[0..j].iter().max().unwrap(),
                    horizontal[j+1..].iter().max().unwrap(),
                ];

            if max_heights.iter().min().unwrap().to_owned() >= &height {
                result -= 1;
            }

        }
    }

    println!("{result:?}");
}
