use anyhow::{Context, anyhow};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

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

struct Index {
    entries: BTreeMap<String, String>,
}

impl Index {
    fn load(index_path: &Path) -> anyhow::Result<Self> {
        let content = fs::read_to_string(index_path).unwrap_or_default();
        let mut entries = BTreeMap::new();

        for line in content.lines() {
            // format: <hash> <path>
            let (hash, path) = match line.split_once(' ') {
                Some(v) => v,
                None => continue,
            };
            if !hash.is_empty() && !path.is_empty() {
                entries.insert(path.to_string(), hash.to_string());
            }
        }

        Ok(Self { entries })
    }

    fn save(&self, index_path: &Path) -> anyhow::Result<()> {
        let out: String = self
            .entries
            .iter()
            .map(|(path, hash)| format!("{hash} {path}\n"))
            .collect();

        fs::write(index_path, out)?;
        Ok(())
    }

    fn add(&mut self, path: String, hash: String) {
        self.entries.insert(path, hash);
    }
}

fn ensure_repo_exists() -> anyhow::Result<()> {
    let repo_dir = Path::new(".rvcs");
    if !repo_dir.exists() {
        return Err(anyhow!("not an rvcs repository (run `rvcs init` first)"));
    }
    let objects_dir = repo_dir.join("objects");
    if !objects_dir.exists() {
        return Err(anyhow!(".rvcs/objects missing (repo seems corrupted)"));
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    hex::encode(digest)
}

fn normalize_path(p: &Path) -> String {
    p.to_string_lossy().replace('\\', "/")
}
