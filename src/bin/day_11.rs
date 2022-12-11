#[derive(Debug)]
struct Monkey {
    activity: u32,
    items: Vec<u32>,
    operation: char,
    operand: u32,
    test: Vec<u32>
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.activity == other.activity
    }
}
impl Eq for Monkey {}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.activity.cmp(&other.activity))
    }
}

impl Ord for Monkey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.activity.cmp(&other.activity)
    }
}

fn main() {
    let binding = include_str!("./day_11.input").replace(",", "").to_string();
    let mut operations: Vec<char> = vec![];
    let input: Vec<Vec<Vec<u32>>> = binding.trim()
        .split("\n\n")
            .map(|x| x.split("\n")
                .skip(1)
                .flat_map(|z| z.split(": ")
                    .skip(1))
                    .map(|y| y
                        .trim()
                        .split(" ")
                        .filter_map(|s| {
                            if ["+", "*"].contains(&s) {
                                operations.push(s.chars().nth(0).unwrap());
                            }
                            s.parse::<u32>().ok()})
                        .collect()
                    )
                 .collect())
            .collect();
     operations.reverse();

    let mut monkeys: Vec<Monkey> = vec![];

    for monkey in input {
        monkeys.push(
            Monkey {
                activity: 0,
                items: monkey[0].to_owned(),
                operation: operations.pop().unwrap(),
                operand: if monkey[1].len() > 0 {
                    monkey[1][0]
                }
                else {
                    0
                },
                test: monkey[2..=4].iter()
                    .map(|x| x[0])
                    .collect() }
        )
    }

    for _ in 0..20 {
        println!("{monkeys:#?}");
        for monkey in 0..monkeys.len() {
            let items = monkeys[monkey].items.clone();
            monkeys[monkey].activity += items.len() as u32;
            for item in items {
                let worry: u32 = match monkeys[monkey].operand {
                    0 => match monkeys[monkey].operation {
                        '+' => item + item,
                        '*' => item * item,
                        _ => 0
                    }
                    _ => match monkeys[monkey].operation {
                        '+' => item + monkeys[monkey].operand,
                        '*' => item * monkeys[monkey].operand,
                        _ => 0
                    }
                };
                let test: usize;
                if (worry / 3) % monkeys[monkey].test[0] == 0 {
                    test = monkeys[monkey].test[1] as usize;
                }
                else {
                    test = monkeys[monkey].test[2] as usize;
                }
                monkeys[test].items.push(worry / 3);
            }
            monkeys[monkey].items = vec![];
        }
    }
    monkeys.sort();

    for i in &monkeys {
        println!("{:?}", i.activity);
    }

    let len = monkeys.len();
    println!("{}", monkeys[len-1].activity*monkeys[len-2].activity)

}
