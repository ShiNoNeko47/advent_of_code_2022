fn main() {
    let input: Vec<char> = include_str!("./day_06.input").trim().chars().collect();

    let mut diff_to_go = 3;
    for i in 0..input.len()-3 {
        if !input[i+1..=i+diff_to_go].contains(&input[i]) {
            if diff_to_go == 0 {
                println!("{}", i+1);
                // for j in i-3..=i {
                //     print!("{:?}", input[j]);
                // }
                break;
            }
            diff_to_go -= 1;
            continue;
        }
        diff_to_go = 3;
    }
}
