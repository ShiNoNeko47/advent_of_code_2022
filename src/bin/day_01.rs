use std::io::stdin;

fn main() {
    let mut vec: Vec<Vec<u32>> = vec![];
    let mut input: String;
    let mut i: usize = 0;
    let mut sum = 0;
    let mut top_3: [u32; 3] = [0, 0, 0];

    vec.push(vec![]);
    loop {
        input = String::new();
        stdin().read_line(&mut input).expect("Input failed");
        if input.trim() == "" {
            if vec[i].len() == 0 {
                vec.pop();
                break;
            }
            sum = vec[i].iter().sum();
            top_3.sort();
            if sum > top_3[0] {
                top_3[0] = sum;
            }

            i = i + 1;
            vec.push(vec![]);
            continue;
        }
        vec[i].push(input.trim().parse().unwrap());
    }
    println!("{}", top_3[0] + top_3[1] + top_3[2]);
}
