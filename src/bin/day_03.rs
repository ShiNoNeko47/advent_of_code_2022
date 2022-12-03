fn main() {
    let input: Vec<&str> = include_str!("./day_03.input")
        .trim()
        .split("\n")
        .collect();

    let mut prios: Vec<u32> = vec![];

    for i in (0..input.len()).step_by(3){
        let mut duplicate: u8 = 0;
        for j in input[i].chars() {
            if input[i+1].contains(j) && input[i+2].contains(j) {
                if j as u8 > 'a' as u8 {
                    duplicate = j as u8 - 96;
                    break;
                }
                duplicate = j as u8 - (64 - 26);
                break;
            }
        }
        prios.push(duplicate as u32);
    }

    let sum: u32 = prios.iter().sum();

    println!("{}", sum);
}
