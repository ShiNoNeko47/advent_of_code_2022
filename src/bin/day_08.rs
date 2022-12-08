fn main() {
    let input: Vec<Vec<u8>> = include_str!("./day_08.input").trim()
        .split("\n")
        .map(|x| x.chars()
             .map(|y| y.to_string().parse().unwrap())
             .collect())
        .collect();

    let x: usize = input[0].len();
    let y: usize = input.len();

    let mut result: usize = 0;

    for i in 1..y-1 {
        for j in 1..x-1 {
            let tree_height: u8 = input[i][j];
            let vertical: &Vec<u8> = &input.iter().map(|x| x[j]).collect();
            let horizontal: &Vec<u8> = &input[i];

            let mut heights: Vec<Vec<u8>> = vec![
                    vertical[0..i].to_vec(),
                    vertical[i+1..].to_vec(),
                    horizontal[0..j].to_vec(),
                    horizontal[j+1..].to_vec(),
                ];

            heights[0].reverse();
            heights[2].reverse();

            let mut scenic_directions: [usize; 4] = [0, 0, 0, 0];

            for k in 0..4 {
                for height in heights[k].iter() {
                    scenic_directions[k] += 1;
                    if height >= &tree_height {
                        break;
                    }
                }
            }

            let scenic_score: usize = scenic_directions.iter().product();

            if scenic_score > result {
                result = scenic_score;
            }

        }
    }

    println!("{result:?}");
}
