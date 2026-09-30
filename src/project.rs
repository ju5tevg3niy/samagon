use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use serde::Deserialize;

use crate::utils;

pub mod assets;

pub type DescVersion = u32;

#[derive(Debug, Deserialize)]
pub struct ProjectDesc {
    version: DescVersion,
    assets: Vec<String>,
}

#[derive(Debug)]
pub struct Project {
    desc: ProjectDesc,
    root_path: PathBuf,
    project_path: PathBuf,
}

const PROJECT_DESC_FILENAME: &str = "project.smgn.json";

impl Project {
    pub fn new<P: AsRef<Path>>(root_path: P) -> anyhow::Result<Self> {
        let root_path = root_path.as_ref();
        if !root_path
            .metadata()
            .with_context(|| format!("Failed to get metadata for {root_path:?}"))?
            .is_dir()
        {
            anyhow::bail!("Project root path is not a directory");
        }

        let project_path = root_path.join(PROJECT_DESC_FILENAME);

        let desc: ProjectDesc = utils::read_json(&project_path)
            .with_context(|| format!("Failed to read project file: {project_path:?}"))?;

        Ok(Self {
            desc,
            root_path: root_path.to_owned(),
            project_path,
        })
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn read_project_desc_from_json() {
        let project_file_path = "misc/test_data/project.smgn.json";

        let project_file_data =
            fs::read_to_string(project_file_path).expect("Failed to read test project file");

        let project_desc: ProjectDesc =
            serde_json::from_str(&project_file_data).expect("Failed to parse test project json");

        assert_eq!(project_desc.version, 2026);
    }

    #[test]
    fn read_project_from_root_path() {
        let project_root_path = "misc/test_data/test_project1";

        let project = Project::new(project_root_path).expect("Failed to read project");

        assert_eq!(project.desc.version, 2026);
        assert_eq!(
            project.desc.assets,
            ["assets/test_guy", "assets/test_sprite",]
        );
    }

    #[test]
    #[should_panic]
    fn read_project_invalid_root_path() {
        Project::new("/").unwrap();
    }
}
