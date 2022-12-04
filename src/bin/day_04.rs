fn main() {
    let input: Vec<Vec<Vec<u8>>> = include_str!("./day_04.input")
        .trim()
        .split("\n")
        .map(|x| x.split(",")
            .map(|y| y.split("-")
                 .map(|z| z.parse().unwrap())
                 .collect())
            .collect())
        .collect();

    let mut result: u16 = 0;

    for x in input {
        if x[0][0] >= x[1][0] && x[0][1] <= x[1][1] || x[0][0] <= x[1][0] && x[0][1] >= x[1][1] {
            result += 1;
        }
    }
    println!("{result}");
}
