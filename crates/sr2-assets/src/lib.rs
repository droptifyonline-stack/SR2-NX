use serde::{Deserialize, Serialize};
use std::path::{Component, Path, PathBuf};
#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub version: u32,
    pub assets: Vec<Asset>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub path: PathBuf,
    pub kind: Kind,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Model,
    Texture,
    Animation,
    Scene,
}
pub fn asset_root() -> PathBuf {
    std::env::var_os("SR2_ASSET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("local-assets"))
}
impl Manifest {
    pub fn validate(&self, root: &Path) -> Result<(), String> {
        if self.version != 1 {
            return Err("unsupported manifest version".into());
        }
        let root = root.canonicalize().map_err(|e| e.to_string())?;
        let mut ids = std::collections::HashSet::new();
        for asset in &self.assets {
            if asset.id.is_empty() || !ids.insert(&asset.id) {
                return Err("empty or duplicate asset id".into());
            }
            if asset
                .path
                .components()
                .any(|c| !matches!(c, Component::Normal(_)))
                || asset.path.as_os_str().is_empty()
            {
                return Err("asset paths must be relative without traversal".into());
            }
            let path = root
                .join(&asset.path)
                .canonicalize()
                .map_err(|e| e.to_string())?;
            if !path.starts_with(&root) || !path.is_file() {
                return Err("asset escapes root or is not a file".into());
            }
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            let supported = match asset.kind {
                Kind::Model | Kind::Animation => matches!(ext, "glb" | "gltf"),
                Kind::Texture => matches!(ext, "png" | "ktx2"),
                Kind::Scene => ext == "json",
            };
            if !supported {
                return Err(format!("unsupported extension for {}", asset.id));
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_traversal() {
        let root = std::env::temp_dir();
        let m = Manifest {
            version: 1,
            assets: vec![Asset {
                id: "x".into(),
                path: "../secret.png".into(),
                kind: Kind::Texture,
            }],
        };
        assert!(m.validate(&root).is_err());
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneDescription {
    pub version: u32,
    pub instances: Vec<SceneInstance>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneInstance {
    pub model: PathBuf,
    pub translation: [f32; 3],
    pub rotation_y: f32,
    pub scale: f32,
}
impl SceneDescription {
    pub fn validate(&self, root: &Path) -> Result<(), String> {
        if self.version != 1 {
            return Err("unsupported scene version".into());
        }
        for (i, item) in self.instances.iter().enumerate() {
            if !item.scale.is_finite()
                || item.scale <= 0.
                || !item.rotation_y.is_finite()
                || item.translation.iter().any(|v| !v.is_finite())
            {
                return Err("invalid scene transform".into());
            }
            Manifest {
                version: 1,
                assets: vec![Asset {
                    id: i.to_string(),
                    path: item.model.clone(),
                    kind: Kind::Model,
                }],
            }
            .validate(root)?;
        }
        Ok(())
    }
}
