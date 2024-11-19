use std::env;

#[derive(Debug)]
pub struct Command {
    pub command: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn collect() -> Self {
        let mut args: Vec<String> = env::args().collect();
        let mut com = String::new();
        let mut arg: Vec<String> = vec![];
        if args.len() > 1 {
            com = args[1].clone();
        }
        if args.len() > 2 {
            arg = args.splice(2.., []).collect();
        }
        Self {
            command: com,
            args: arg,
        }
    }
}
