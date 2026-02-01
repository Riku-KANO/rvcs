use anyhow::{Context, anyhow};
use std::fs;
use std::path::{Path, PathBuf};

use crate::index::Index;
use crate::utils::{ensure_repo_exists, normalize_path, sha256_hex};

/// Add a file to the repository
///
/// # Arguments
///
/// * `path` - Path to the file to add
///
/// # Errors
///
/// * `anyhow::Error` - If the file does not exist
/// * `anyhow::Error` - If the file is a directory
///
pub fn run(path: String) -> anyhow::Result<()> {
    ensure_repo_exists()?;

    let input_path = PathBuf::from(path);
    if !input_path.exists() {
        return Err(anyhow!("path does not exist: {}", input_path.display()));
    }
    if input_path.is_dir() {
        return Err(anyhow!(
            "directory is not supported yet: {}",
            input_path.display()
        ));
    }

    let bytes = fs::read(&input_path)
        .with_context(|| format!("failed to read {}", input_path.display()))?;

    let hash = sha256_hex(&bytes);
    let obj_path = Path::new(".rvcs").join("objects").join(&hash);

    if !obj_path.exists() {
        fs::write(&obj_path, &bytes)
            .with_context(|| format!("failed to write object {}", obj_path.display()))?;
    }

    let index_path = Path::new(".rvcs").join("index");
    let mut index = Index::load(&index_path)?;
    let key_path = normalize_path(&input_path);

    index.add(key_path, hash.clone());
    index.save(&index_path)?;

    println!("added {}  {}", hash, input_path.display());
    Ok(())
}
