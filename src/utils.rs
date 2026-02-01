use anyhow::anyhow;
use sha2::{Digest, Sha256};
use std::path::Path;

pub fn ensure_repo_exists() -> anyhow::Result<()> {
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

pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    hex::encode(digest)
}

pub fn normalize_path(p: &Path) -> String {
    p.to_string_lossy().replace('\\', "/")
}
