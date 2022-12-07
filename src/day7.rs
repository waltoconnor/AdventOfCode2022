//AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
//what a disaster

#[derive(Clone, Debug)]
struct File {
    pub name: String,
    pub size: u32
}

impl File {
    pub fn new(name: String, size: u32) -> Self {
        File { name: name, size: size }
    }
}

#[derive(Clone, Debug)]
struct Directory {
    pub name: String,
    pub files: Vec<File>,
    pub subdirs: Vec<Directory>,
    pub size: u32
}

impl Directory {
    pub fn new(name: String) -> Self {
        Directory { name: name, files: Vec::new(), subdirs: Vec::new(), size: 0 }
    }

    pub fn get_size(&mut self) -> u32 {
        let size = self.files.iter().map(|f| f.size).sum::<u32>() + self.subdirs.iter_mut().map(|sd| sd.get_size()).sum::<u32>();
        self.size = size;
        size
    }

    pub fn search_pt1(&self, max_size: u32) -> u32 {
        let mut total = self.subdirs.iter().map(|s| s.search_pt1(max_size)).sum::<u32>();
        if self.size <= max_size {
            total += self.size;
        }
        return total
    }

    pub fn get_folders_and_sizes(&self) -> Vec<(String, u32)> {
        let mut v = Vec::from([(self.name.clone(), self.size)]);
        v.extend(self.subdirs.iter().map(|s| s.get_folders_and_sizes()).flatten());
        v
    }

    pub fn get_folder(&mut self, path: Vec<&str>) -> &mut Directory {
        println!("GET FOLDER PATH = {:?}", path);
        if path[0] == "" && path.len() == 1 {
            return self;
        }

        if path.len() == 1 {
            println!("Searching {:?} for {}", self.subdirs, path[0].replace("/", ""));
            self.subdirs.iter_mut().filter(|sd| sd.name == String::from(path[0].replace("/", ""))).next().unwrap()
        }
        else {
            let child = path[1];
            let rest = Vec::from(&path[1..]);
            self.subdirs.iter_mut().filter(|sd| sd.name == String::from(path[0])).next().unwrap().get_folder(rest)

        }
    }
}

#[derive(Clone, Debug)]
enum Command {
    LS((Vec<Directory>, Vec<File>)),
    CD(String)
}

fn parse_command(cmd: &str) -> Option<Command> {
    //println!("cmd: {}", cmd);
    if cmd == "" { return None; }
    let lines = cmd.lines().into_iter().collect::<Vec<&str>>();
    let head = lines.first().unwrap();
    let tail = if lines.len() > 1 { &lines[1..] } else { &[] };
    let split = head.trim_start().split(" ").into_iter().collect::<Vec<&str>>();
    let (cmd_text, cmd_arg) = match split.as_slice() {
        [ct, ca, ..] => (ct, ca),
        [ct] => (ct, &""),
        _ => panic!("Couldn't parse command line")
    };

    //println!("cmd: {}, arg: {}", cmd_text, cmd_arg);

    if *cmd_text == "ls" {
        let mut dirs = Vec::new();
        let mut files = Vec::new();
        for line in tail.iter() {
            // println!("line: {}", line);
            match line.split(" ").collect::<Vec<&str>>().as_slice() {
                ["dir", name] => { dirs.push(Directory::new(String::from(*name))); },
                [num, name] => { files.push(File::new(String::from(*name), num.parse().unwrap())); },
                _ => panic!("Could not parse ls result line")
            }
        }
        Some(Command::LS((dirs, files)))
    }
    else {
        Some(Command::CD(String::from(*cmd_arg)))
    }

}

fn split_to_commands(input: &str) -> Vec<Command> {
    input.split("$").map(|cmd| parse_command(cmd)).filter_map(|l| l).collect()
}

fn build_file_structure(cmds: &Vec<Command>) -> Directory {
    let mut root = Directory::new(String::from("root"));
    let mut cur_path = String::from("");
    for idx in 0..cmds.len() {
        let cmd = cmds[idx].clone();
        println!("CMD: {:?}, cur_path: {}", cmd, cur_path);
        match cmd{
            Command::CD(path) => {
                if path == "/" {
                    cur_path = String::from("");
                }
                else if path == ".." {
                    cur_path = match cur_path.rfind("/") {
                        Some(n) => cur_path[..n].to_string(),
                        None => String::from("")
                    };
                    
                }
                else {
                    if cur_path != "" {
                        cur_path += "/";
                    }
                    
                    cur_path += &path;
                }
            },
            Command::LS((dirs, files)) => {
                let cur_folder = root.get_folder(cur_path.split("/").collect());
                cur_folder.files = files;
                cur_folder.subdirs = dirs;
            }

        }
    }
    root
}

fn main() {
    let input = include_str!("../input/day7.txt");
    let commands = split_to_commands(input);
    let mut structure = build_file_structure(&commands);
    structure.get_size();
    println!("{:#?}", structure);
    println!("A: {}", structure.search_pt1(100000));

    let needed_space: i32 = 30000000 - (70000000 - structure.size as i32);
    println!("Needed space: {}", needed_space);
    let folders_sizes = structure.get_folders_and_sizes();
    let min_size = folders_sizes.iter().map(|(_, size)| size).filter(|size| (**size as i32) >= needed_space).min().unwrap();
    println!("B: {}", min_size);

}