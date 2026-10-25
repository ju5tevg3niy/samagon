use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use serde::Deserialize;

use crate::constants::PROJECT_ASSETS_MANIFEST_FILENAME;
use crate::constants::PROJECT_ASSETS_PATH;
use crate::constants::PROJECT_ROOT_FILENAME;
use crate::utils;

pub mod assets;

use assets::AssetsManifest;

type DescVersion = u32;

#[derive(Debug, Deserialize)]
pub struct ProjectDesc {
    pub version: DescVersion,
}

#[derive(Debug)]
pub struct Project {
    pub desc: ProjectDesc,
    pub root_path: PathBuf,
    pub project_path: PathBuf,
    pub assets_path: PathBuf,
    pub assets_manifest: AssetsManifest,
}

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

        let project_path = root_path.join(PROJECT_ROOT_FILENAME);

        let desc: ProjectDesc = utils::read_json(&project_path)
            .with_context(|| format!("Failed to read project file: {project_path:?}"))?;

        let assets_path = root_path.join(PROJECT_ASSETS_PATH);

        let assets_manifest_path = assets_path.join(PROJECT_ASSETS_MANIFEST_FILENAME);

        let assets_manifest = AssetsManifest::read(assets_manifest_path)
            .context("Failed to laod project's asset manifest")?;

        Ok(Self {
            desc,
            root_path: root_path.to_owned(),
            project_path,
            assets_path,
            assets_manifest,
        })
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn read_project_desc_from_json() {
        let project_file_path = "misc/test_data/project2026.smgn.json";

        let project_file_data =
            fs::read_to_string(project_file_path).expect("Failed to read test project file");

        let project_desc: ProjectDesc =
            serde_json::from_str(&project_file_data).expect("Failed to parse test project json");

        assert_eq!(project_desc.version, 2026);
    }

    #[test]
    fn read_project_from_root_path() {
        let project_root_path = "misc/test_data/test_project2026";

        let project = Project::new(project_root_path).expect("Failed to read project");

        assert_eq!(project.desc.version, 2026);
        assert_eq!(project.assets_manifest.files, ["test_guy", "test_sprite",]);
    }

    #[test]
    #[should_panic]
    fn read_project_invalid_root_path() {
        Project::new("/").unwrap();
    }

    #[test]
    fn read_assets_manifest() {
        let assets_manifest_path = "misc/test_data/manifest2026.smgn.json";

        let assets_manifest = AssetsManifest::read(assets_manifest_path)
            .expect("Failed to read test asset manifest file");

        assert_eq!(assets_manifest.version, 2026);
        assert_eq!(assets_manifest.files, ["test", "asset", "files"]);
    }
}
