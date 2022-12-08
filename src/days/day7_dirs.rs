use itertools::Itertools;

#[derive(Debug)]
enum Node<'a> {
    Dir(&'a str),
    File(&'a str, u32)
}

#[derive(Debug)]
enum Command<'a> {
    Cd(&'a str),
    Ls
}

type Output<'a> = Vec<(Command<'a>, Vec<Node<'a>>)>;

fn parse(output: &str) -> Output {
    let commands = output
        .split('$')
        .filter(|c| c != &"")
        .map(|command| {
            let mut command_lines = command.split('\n').filter(|c| c != &"").rev().collect_vec();
            let command = match command_lines.pop().expect("empty command").trim() {
                "ls" => Command::Ls,
                cd => Command::Cd(cd.split(' ').nth(1).expect("Missing cd arg"))
            };
            let nodes = command_lines.into_iter().map(|cl| {
                let ls_line = cl.split(' ').next_tuple::<(_, _)>().expect("Unexpected command output");
                match ls_line {
                    ("dir", name) => Node::Dir(name),
                    (size, name) => Node::File(name, size.parse::<u32>().expect("Not u32 size"))
                }
            }).collect_vec();
            return (command, nodes)
        })
        .collect_vec();
    println!("{commands:?}");
    commands
}


type Fs<'a> = Vec<(String, Vec<Node<'a>>)>;

fn find_dir_size(fs: &Fs, path: &str) -> u32 {
    fs.iter().map(|(p, nodes)| {
        if p.starts_with(&path) {
            nodes.iter().map(|n| match n {
                Node::File(_, size) => *size,
                _ => 0,
            }).sum()
        } else { 0 }
    }).sum()
}

fn creat_fs(output: Output) -> Fs {
    let mut current_path = vec![];
    let mut fs = vec![];
    for (command, nodes) in output {
        match command {
            Command::Cd("..") => { current_path.pop(); },
            Command::Cd("/") => { current_path.clear(); },
            Command::Cd(folder) => { current_path.push(folder); },
            Command::Ls => {
                fs.push((current_path.join("/"), nodes));
            }
        }
    }
    println!("---------------------");
    println!("{fs:?}");
fs
    // println!("{fs:?}")
}

pub fn large_dirs(output: &str) -> u32 {
    let fs = creat_fs(parse(output));
    fs.iter().map(|(path, _)| {
        // println!("{path:?}: {}", find_dir_size(&fs, path));
        find_dir_size(&fs, path)
    }).filter(|s| s <= &100000).sum()
}

pub fn large_enough_dir(output: &str) -> u32 {
    let fs = creat_fs(parse(output));
    let free_space = 70000000 - find_dir_size(&fs, "");
    let requried = 30000000 - free_space;
    fs.iter().map(|(path, _)| {
        println!("{path:?}: {}", find_dir_size(&fs, path));
        find_dir_size(&fs, path)
    }).filter(|s| s >= &requried).min().expect("No large enough folder")
}

#[cfg(test)]
mod tests {
    use crate::inputs;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(large_dirs(&inputs::demo_input(7)), 95437);
        assert_eq!(large_enough_dir(&inputs::demo_input(7)), 24933642);
        assert_eq!(large_dirs(&inputs::task_input(7)), 1582412);

        // assert_eq!(large_enough_dir(&inputs::task_input(7)), 31148261);
        // That's not the right answer; your answer is too high. If you're stuck, make sure you're using the full input data; there are also some general tips on the about page, or you can ask for hints on the subreddit. Please wait one minute before trying again. (You guessed 31148261.) [Return to Day 7]
        assert_eq!(large_enough_dir(&inputs::task_input(7)) < 31148261, true);
        assert_eq!(large_enough_dir(&inputs::task_input(7)), 3696336);
    
    }
}
