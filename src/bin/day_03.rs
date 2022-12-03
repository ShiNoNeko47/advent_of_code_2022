fn main() {
    let input: Vec<(&str, &str)> = include_str!("./day_03.input")
        .trim()
        .split("\n")
        .map(|x| x.split_at(x.len()/2))
        .collect();

    let mut prios: Vec<u32> = vec![];

    for i in input {
        let mut duplicate: u8 = 0;
        for j in i.0.chars() {
            for k in i.1.chars() {
                if j == k {
                    if j as u8 > 'a' as u8 {
                        duplicate = j as u8 - 96;
                        break;
                    }
                    duplicate = j as u8 - (64 - 26);
                }
            }
            if duplicate != 0 {
                prios.push(duplicate as u32);
                break;
            }
        }
    }
    println!("{:?}", prios);
    let sum: u32 = prios.iter().sum();

    println!("{}", sum);
}
