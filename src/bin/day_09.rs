use std::io::stdin;

#[derive(Debug, Clone, Copy)]
struct Position {
    x: isize,
    y: isize
}

impl PartialEq for Position{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn main() {
    let input: Vec<Vec<char>> = include_str!("./day_09.input").trim()
        .split("\n")
        .map(|x| x.trim()
             .split(" ").collect())
        .map(|mut x: Vec<&str>| {
            let n: usize = x.pop().unwrap().parse().unwrap();
            let direction: &str = x.pop().unwrap();
            for _ in 0..n {
                x.push(direction);
            }
            return x.iter().map(|y| y.chars().nth(0).unwrap()).collect();
        })
    .collect();

    let input: Vec<char> = input.into_iter().flat_map(|x| x).collect();

    let mut rope: Vec<Position> = vec![Position { x: 0, y: 0 }; 10];

    let mut visited: Vec<Position> = vec![Position { x: 0, y: 0 }];

    for direction in input {
        match direction {
            'U' => {
                rope[0].y += 1;
            },
            'D' => {
                rope[0].y -= 1;
            },
            'L' => {
                rope[0].x -= 1;
            },
            'R' => {
                rope[0].x += 1;
            },
            _ => panic!("invalid direction")
        }
        for i in 1..10 {
            if rope[i-1].y > rope[i].y && rope[i-1].x > rope[i].x && (rope[i-1].x > rope[i].x+1 || rope[i-1].y > rope[i].y+1) {
                rope[i].y += 1;
                rope[i].x += 1;
            }
            else if rope[i-1].y < rope[i].y && rope[i-1].x > rope[i].x && (rope[i-1].x > rope[i].x+1 || rope[i-1].y < rope[i].y-1) {
                rope[i].y -= 1;
                rope[i].x += 1;
            }
            else if rope[i-1].y < rope[i].y && rope[i-1].x < rope[i].x && (rope[i-1].x < rope[i].x-1 || rope[i-1].y < rope[i].y-1) {
                rope[i].y -= 1;
                rope[i].x -= 1;
            }
            else if rope[i-1].y > rope[i].y && rope[i-1].x < rope[i].x && (rope[i-1].x < rope[i].x-1 || rope[i-1].y > rope[i].y+1) {
                rope[i].y += 1;
                rope[i].x -= 1;
            }
            else if rope[i-1].y > rope[i].y+1 {
                rope[i].y += 1;
            }
            else if rope[i-1].y < rope[i].y-1 {
                rope[i].y -= 1;
            }
            else if rope[i-1].x > rope[i].x+1 {
                rope[i].x += 1;
            }
            else if rope[i-1].x < rope[i].x-1 {
                rope[i].x -= 1;
            }
        }
        if !visited.contains(&rope[9]) {
            visited.push(rope[9].clone());
        }
    }

    println!("{}", visited.len());
}
