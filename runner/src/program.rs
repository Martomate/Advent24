use std::process::Command;

pub struct Program {
    cmd: String,
    args: Vec<String>,
}

impl Program {
    pub fn new(cmd: &str) -> Self {
        Self {
            cmd: cmd.to_string(),
            args: Vec::new(),
        }
    }

    pub fn with_args(mut self, args: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        for arg in args {
            self.args.push(arg.as_ref().to_string());
        }
        self
    }
}

impl From<&Program> for Command {
    fn from(val: &Program) -> Self {
        let mut cmd = Command::new(&val.cmd);
        for arg in val.args.iter() {
            cmd.arg(arg);
        }
        cmd
    }
}
