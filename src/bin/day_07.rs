#[derive(Debug)]
struct Directory {
    name: String,
    size: u32
}

impl PartialEq for Directory {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn main() {
    let input: Vec<&str> = include_str!("./day_07.input").trim().split("\n").collect();

    let mut result: u32 = 0;

    let mut dir_chain: Vec<Directory> = vec![];
    
    for i in input {

        if i == "$ cd .." {
            let size: u32 = dir_chain.pop().unwrap().size;
            if size <= 100000 {
                result += size;
            }
        }
        else if i.starts_with("$ cd") {
            let dir: Directory = Directory { name: i.split(" ").last().unwrap().to_owned(), size: 0 };
            dir_chain.push(dir);
        }
        else if !i.starts_with("$") {
            if i.split(" ").nth(0).unwrap().parse::<u32>().is_ok() {
                for j in 0..dir_chain.len() {
                    dir_chain[j].size += i.split(" ").nth(0).unwrap().parse::<u32>().unwrap();
                }
            }
        }
    }
    println!("{result}");
}
