fn main() {
    let input: Vec<Vec<u32>> = include_str!("./day_18.input").trim()
        .split("\n")
        .map(|x| x.split(",")
             .filter_map(|y| y.parse().ok())
             .collect())
        .collect();

    let mut surface_area: u32 = input.len() as u32 * 6 as u32;

    for cube in input.iter() {
        for cube_other in input.iter() {
            let mut diff: i32 = 0;
            if cube == cube_other {
                continue;
            }
            for i in 0..3 {
                let diff_i: i32 = cube[i] as i32 - cube_other[i] as i32;
                if diff_i == 0 {
                    continue;
                }
                else if diff == 0 && diff_i.abs() == 1 {
                    diff = diff_i;
                }
                else if diff != 0 {
                    diff = 0;
                    break;
                }
                else {
                    break;
                }
            }
            if diff.abs() == 1 {
                surface_area -= 1;
            }
        }
    }

    println!("{surface_area:#?}");
}
