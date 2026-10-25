use std::path::Path;
use std::path::PathBuf;

use crate::utils;

use anyhow::Context;
use serde::Deserialize;

use super::DescVersion;

#[derive(Debug)]
pub enum AssetType {
    Invalid(String),
    Sprite,
}

impl<S: AsRef<str>> From<S> for AssetType {
    fn from(value: S) -> Self {
        match value.as_ref() {
            "sprite" => Self::Sprite,
            other => Self::Invalid(other.into()),
        }
    }
}

#[derive(Debug)]
pub enum AssetFormat {
    Invalid(String),
    Png,
}

impl<S: AsRef<str>> From<S> for AssetFormat {
    fn from(value: S) -> Self {
        match value.as_ref() {
            "png" => Self::Png,
            other => Self::Invalid(other.into()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AssetDesc {
    pub version: DescVersion,
    pub asset_type: String,
    pub asset_format: String,
}

#[derive(Debug)]
pub struct Asset {
    pub asset_desc: AssetDesc,
    pub asset_path: PathBuf,
    pub asset_type: AssetType,
    pub asset_format: AssetFormat,
}

impl Asset {
    pub fn read<P: AsRef<Path>>(asset_path: P) -> anyhow::Result<Self> {
        let asset_path = asset_path.as_ref();

        let asset_desc: AssetDesc = utils::read_json(asset_path)
            .with_context(|| format!("Failed to read asset file: {asset_path:?}"))?;

        let asset_type = AssetType::from(&asset_desc.asset_type);

        let asset_format = AssetFormat::from(&asset_desc.asset_format);

        Ok(Self {
            asset_desc,
            asset_path: asset_path.to_owned(),
            asset_type,
            asset_format,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct AssetsManifest {
    pub version: DescVersion,
    pub files: Vec<String>,
}

impl AssetsManifest {
    pub fn read<P: AsRef<Path>>(asset_manifest_path: P) -> anyhow::Result<Self> {
        let asset_manifest_path = asset_manifest_path.as_ref();

        utils::read_json(asset_manifest_path)
            .with_context(|| format!("Failed to read asset manifest file: {asset_manifest_path:?}"))
    }
}
