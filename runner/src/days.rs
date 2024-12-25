use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::{
    config::{AdventConfig, RunConfig},
    project::{Project, TemporaryArtifact},
};
use anyhow::{bail, Context};
use kdl::KdlDocument;

pub fn run(day: u8) -> anyhow::Result<()> {
    let (root_dir, run_config) = read_config(day)?;

    if run_config.clean.len() > 1 {
        bail!("multiple clean steps are not supported yet");
    }

    let p = Project::at_root(root_dir.clone());

    p.run_build_steps(run_config.build)?;

    let run_tests = || {
        if let Some(test_program) = run_config.test {
            println!();
            p.run_tests(test_program, &root_dir.join(".advent/testcases"))?;
        }
        Ok(())
    };

    if let Some(artifact) = run_config.clean.first().map(|file| root_dir.join(file)) {
        TemporaryArtifact::defer_deletion(artifact, || {
            let res = run_tests();
            println!();
            res
        })?;
    } else {
        run_tests()?;
    };

    Ok(())
}

fn read_config(day: u8) -> Result<(PathBuf, RunConfig), anyhow::Error> {
    let advent_cfg_path = PathBuf::from(".advent.kdl");
    let advent_config = read_kdl_file(&advent_cfg_path)
        .and_then(|cfg| AdventConfig::try_from(cfg).context("parsing config"))
        .with_context(|| {
            format!(
                "reading advent config at: {:?}",
                advent_cfg_path.as_os_str()
            )
        })?;
    let Some(day_config) = advent_config.days.get(&day) else {
        bail!("could not find day {day} in {advent_cfg_path:?}");
    };

    let run_cfg_path = day_config.root.join(".advent/run.kdl");
    let run_config = read_kdl_file(&run_cfg_path)
        .and_then(|cfg| RunConfig::try_from(cfg).context("parsing config"))
        .with_context(|| format!("reading run config at: {:?}", run_cfg_path.as_os_str()))?;

    Ok((day_config.root.clone(), run_config))
}

fn read_kdl_file(path: impl AsRef<Path>) -> anyhow::Result<KdlDocument> {
    let file = File::open(path.as_ref()).context("opening file")?;
    let content = read_entire_file(file).context("reading file")?;
    let document = KdlDocument::parse(&content).context("parsing kdl")?;
    Ok(document)
}

fn read_entire_file(mut file: File) -> Result<String, std::io::Error> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
