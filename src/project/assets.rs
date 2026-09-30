use std::path::Path;
use std::path::PathBuf;

use crate::utils;

use anyhow::Context;
use serde::Deserialize;

#[derive(Debug)]
pub enum AssetType {
    Invalid,
    Sprite,
}

impl<S: AsRef<str>> From<S> for AssetType {
    fn from(value: S) -> Self {
        match value.as_ref() {
            "sprite" => Self::Sprite,
            _other => Self::Invalid,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AssetDesc {
    version: super::DescVersion,
    asset_type: String,
}

#[derive(Debug)]
pub struct Asset {
    asset_desc: AssetDesc,
    asset_path: PathBuf,
    asset_type: AssetType,
}

impl Asset {
    pub fn read<P: AsRef<Path>>(asset_path: P) -> anyhow::Result<Self> {
        let asset_path = asset_path.as_ref();

        let asset_desc: AssetDesc = utils::read_json(asset_path)
            .with_context(|| format!("Failed to read asset file: {asset_path:?}"))?;

        let asset_type = AssetType::from(&asset_desc.asset_type);

        Ok(Self {
            asset_desc,
            asset_path: asset_path.to_owned(),
            asset_type,
        })
    }
}
