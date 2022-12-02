fn main() {
    let mut input: Vec<Vec<u8>> = include_str!("./day_02.input")
        .split("\n")
        .map(|x| x.split(" ")
             .flat_map(|y| y.chars().map(|z| z as u8))
             .collect())
        .collect();
    input.pop();

    let mut score: i32 = 0;

    for i in 0..input.len() {
        let opponent: i8 = (input[i][0] - 64) as i8;
        let player: i8 = (input[i][1] - 87) as i8;
        match player {
            1 => {
                score += 0;
                let mut opponent = opponent - 1;
                if opponent < 1 {
                    opponent += 3;
                }
                score += opponent as i32;
            },
            2 => {
                score += 3;
                score += opponent as i32;
            },
            3 => {
                score += 6;
                let mut opponent = opponent - 2;
                if opponent < 1 {
                    opponent += 3;
                }
                score += opponent as i32;
            },
            _ => println!("Error")
        }
    }

    println!("{score}");
}
