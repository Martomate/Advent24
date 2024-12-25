use std::{collections::HashMap, path::PathBuf};

use crate::program::Program;
use anyhow::{bail, Context};
use kdl::{KdlDocument, KdlNode};

#[derive(Debug, Clone, PartialEq)]
pub struct AdventConfig {
    pub days: HashMap<u8, DayConfig>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DayConfig {
    pub root: PathBuf,
}

impl DayConfig {
    fn new(root: PathBuf) -> Self {
        Self { root }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RunConfig {
    pub build: Vec<Program>,
    pub clean: Vec<PathBuf>,
    pub test: Option<Program>,
}

impl TryFrom<KdlDocument> for AdventConfig {
    type Error = anyhow::Error;

    fn try_from(config: KdlDocument) -> Result<Self, Self::Error> {
        let days = config
            .get("days")
            .context("could not find toplevel 'nodes' tag")?
            .children()
            .context("the 'nodes' tag must have children")?
            .nodes()
            .iter()
            .map(parse_day)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(AdventConfig {
            days: HashMap::from_iter(days),
        })
    }
}

fn parse_day(c: &KdlNode) -> Result<(u8, DayConfig), anyhow::Error> {
    let root = c
        .get("root")
        .context("a 'day' must have a 'root' property")?
        .as_string()
        .context("the 'root' property must be a string")?;

    let day = c
        .entries()
        .iter()
        .find(|e| e.name().is_none())
        .context("found 'day' without a number")?
        .value()
        .as_integer()
        .context("the number of the day must be an integer")?;
    let day: u8 = day.try_into()?;

    Ok((day, DayConfig::new(PathBuf::from(root))))
}

impl TryFrom<KdlDocument> for RunConfig {
    type Error = anyhow::Error;

    fn try_from(config: KdlDocument) -> Result<Self, Self::Error> {
        let mut build: Vec<Program> = Vec::new();
        let mut clean: Vec<PathBuf> = Vec::new();
        let mut test: Option<Program> = None;

        for n in config.nodes() {
            match n.name().value() {
                "build" => {
                    if let Some(children) = n.children() {
                        for b in children.nodes() {
                            match b.name().value() {
                                "run" => {
                                    let (cmd, args) = b
                                        .entries()
                                        .split_first()
                                        .context("found 'run' without parameters")?;
                                    let args = args
                                        .iter()
                                        .map(|arg| {
                                            arg.value()
                                                .as_string()
                                                .context("the arguments of 'run' must be strings")
                                        })
                                        .collect::<Result<Vec<_>, _>>()?;

                                    let p = Program::new(
                                        cmd.value()
                                            .as_string()
                                            .context("the arguments of 'run' must be strings")?,
                                    )
                                    .with_args(args);
                                    build.push(p);
                                }
                                name => bail!("unknown command '{name}' inside 'build' tag"),
                            }
                        }
                    }
                }
                "clean" => {
                    if let Some(children) = n.children() {
                        for b in children.nodes() {
                            match b.name().value() {
                                "delete" => {
                                    let (file, extra_args) = b
                                        .entries()
                                        .split_first()
                                        .context("found 'delete' without parameters")?;
                                    if !extra_args.is_empty() {
                                        bail!("there may only be one argument to 'delete'");
                                    }
                                    let file = file
                                        .value()
                                        .as_string()
                                        .context("the parameter to 'delete' must be a string")?;
                                    clean.push(PathBuf::from(file));
                                }
                                name => bail!("unknown command '{name}' inside 'clean' tag"),
                            }
                        }
                    }
                }
                "test" => {
                    if let Some(children) = n.children() {
                        for b in children.nodes() {
                            match b.name().value() {
                                "run" => {
                                    let (cmd, args) = b
                                        .entries()
                                        .split_first()
                                        .context("found 'run' without parameters")?;
                                    let args = args
                                        .iter()
                                        .map(|arg| {
                                            arg.value()
                                                .as_string()
                                                .context("the arguments of 'run' must be strings")
                                        })
                                        .collect::<Result<Vec<_>, _>>()?;

                                    let p = Program::new(
                                        cmd.value()
                                            .as_string()
                                            .context("the arguments of 'run' must be strings")?,
                                    )
                                    .with_args(args);

                                    if test.is_some() {
                                        bail!("multiple 'run' tags are not supported in 'test'");
                                    }
                                    test = Some(p);
                                }
                                name => bail!("unknown command '{name}' inside 'build' tag"),
                            }
                        }
                    }
                }
                name => bail!("unknown command '{name}' inside 'day' tag"),
            }
        }

        Ok(Self { build, clean, test })
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use kdl::KdlDocument;

    use super::*;

    #[test]
    fn parse_advent_config() {
        let s = r#"
        days {
            day 5 root="some/path"
            day 12 root="another/path"
        }
        "#;
        let kdl = KdlDocument::parse(s).unwrap();
        let config = AdventConfig::try_from(kdl).unwrap();
        assert_eq!(
            config,
            AdventConfig {
                days: HashMap::from_iter([
                    (5, DayConfig::new(PathBuf::from("some/path"))),
                    (12, DayConfig::new(PathBuf::from("another/path"))),
                ]),
            }
        );
    }

    #[test]
    fn parse_run_config() {
        let s = r#"
        build {
            run roc build main.roc
        }
        clean {
            delete main
        }
        test {
            run main
        }
        "#;
        let kdl = KdlDocument::parse(s).unwrap();
        let config = RunConfig::try_from(kdl).unwrap();
        assert_eq!(
            config,
            RunConfig {
                build: vec![Program::new("roc").with_args(["build", "main.roc"])],
                clean: vec![PathBuf::from("main")],
                test: Some(Program::new("main")),
            }
        );
    }
}
