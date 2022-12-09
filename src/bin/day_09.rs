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

    let mut head: Position = Position { x: 0, y: 0 };
    let mut tail: Position = Position { x: 0, y: 0 };

    let mut visited: Vec<Position> = vec![Position { x: 0, y: 0 }];

    for direction in input {
        match direction {
            'U' => {
                head.y += 1;
                if head.y == tail.y+2 {
                    tail.y += 1;
                    tail.x = head.x;
                }
            },
            'D' => {
                head.y -= 1;
                if head.y == tail.y-2 {
                    tail.y -= 1;
                    tail.x = head.x;
                }
            },
            'L' => {
                head.x -= 1;
                if head.x == tail.x-2 {
                    tail.x -= 1;
                    tail.y = head.y;
                }
            },
            'R' => {
                head.x += 1;
                if head.x == tail.x+2 {
                    tail.x += 1;
                    tail.y = head.y;
                }
            },
            _ => panic!("invalid direction")
        }
        if !visited.contains(&tail) {
            visited.push(tail.clone());
        }
    }

    println!("{}", visited.len());

    // println!("{input:?}");
}
