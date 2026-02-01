use anyhow::{Context, anyhow};
use std::fs;
use std::path::Path;
use std::time::SystemTime;

use crate::index::Index;
use crate::utils::{ensure_repo_exists, sha256_hex};

pub fn run(message: String) -> anyhow::Result<()> {
    ensure_repo_exists()?;

    let repo_dir = Path::new(".rvcs");
    let objects_dir = repo_dir.join("objects");
    let index_file = repo_dir.join("index");
    let index = Index::load(&index_file)?;

    if index.entries.is_empty() {
        return Err(anyhow!(
            "nothing to commit (create/copy files and use \"rvcs add\" to track)"
        ));
    }

    // 1. Create Tree Object
    let mut tree_content = String::new();
    for (path, hash) in &index.entries {
        tree_content.push_str(&format!("blob {} {}\n", hash, path));
    }

    let tree_hash = sha256_hex(tree_content.as_bytes());
    let tree_path = objects_dir.join(&tree_hash);
    if !tree_path.exists() {
        fs::write(&tree_path, &tree_content).context("failed to write tree object")?;
    }

    // 2. Read HEAD to find parent
    let head_path = repo_dir.join("HEAD");
    let head_content = fs::read_to_string(&head_path).unwrap_or_default();
    let parent_hash = if head_content.starts_with("ref: ") {
        let ref_path_str = head_content.trim().trim_start_matches("ref: ");
        let ref_path = repo_dir.join(ref_path_str);
        if ref_path.exists() {
            Some(fs::read_to_string(ref_path)?.trim().to_string())
        } else {
            None
        }
    } else {
        // Detached HEAD or invalid, handle as detached if it looks like a hash, otherwise None
        // For simplicity, we assume we are always on a branch or empty
        None
    };

    // 3. Create Commit Object
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let mut commit_content = String::new();
    commit_content.push_str(&format!("tree {}\n", tree_hash));
    if let Some(parent) = parent_hash {
        commit_content.push_str(&format!("parent {}\n", parent));
    }
    commit_content.push_str("author rvcs\n");
    commit_content.push_str(&format!("timestamp {}\n", timestamp));
    commit_content.push('\n');
    commit_content.push_str(&message);
    commit_content.push('\n');

    let commit_hash = sha256_hex(commit_content.as_bytes());
    let commit_path = objects_dir.join(&commit_hash);
    if !commit_path.exists() {
        fs::write(&commit_path, &commit_content).context("failed to write commit object")?;
    }

    // 4. Update HEAD ref
    if head_content.starts_with("ref: ") {
        let ref_path_str = head_content.trim().trim_start_matches("ref: ");
        let ref_path = repo_dir.join(ref_path_str);

        // Ensure parent directory for ref exists (e.g. refs/heads)
        if let Some(parent) = ref_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&ref_path, &commit_hash)?;
    } else {
        // Detached HEAD, update HEAD directly? For now, just warn or update if it was pointing to nothing
        // Or maybe we treat HEAD as the ref file if it's not a symbolic ref.
        // But init sets "ref: refs/heads/main"
        fs::write(&head_path, &commit_hash)?; // Fallback
    }

    println!("[{}] {}", &commit_hash[..7], message);

    Ok(())
}
