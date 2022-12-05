fn main() {
    let input: Vec<Vec<&str>> = include_str!("./day_05.input")
        .trim_end()
        .split("\n\n")
        .map(|x| x.split("\n")
             .collect())
        .collect();

    let instructions: Vec<Vec<usize>> = input[1]
        .iter()
        .map(|x| x.split(' ')
             .filter_map(|y| y.parse().ok())
             .collect())
        .collect();

    let mut crates: Vec<Vec<char>> = vec![];

    let mut k: usize = 0;
    for i in (1..=34).step_by(4) {
        crates.push(vec![]);
        for j in 0..8 {
            let current_crate: char = input[0][j].chars().nth(i).unwrap();
            if current_crate != ' ' {
                crates[k].push(current_crate);
            }
        }
        crates[k].reverse();
        k += 1;
    }

    for instruction in instructions {
        // println!("{crates:?}");
        let length = crates[instruction[1]-1].len();
        let mut stack: Vec<char> = crates[instruction[1]-1].drain((length-instruction[0])..).collect();
        stack.reverse();
        crates[instruction[2]-1].append(&mut stack);
        for i in &crates {
            println!("{i:?}");
        }
        println!("");
    }


    for i in crates {
        print!("{}", i.last().unwrap());
    }
}
