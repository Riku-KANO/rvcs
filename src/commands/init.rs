use std::fs;
use std::path::Path;
use anyhow::Ok;

pub fn run() -> anyhow::Result<()> {
    let repo_dir = Path::new(".rvcs");
    let objects_dir = repo_dir.join("objects");
    let index_file = repo_dir.join("index");
    let head_file = repo_dir.join("HEAD");

    if repo_dir.exists() {
        println!(".rvcs already exists");
        return Ok(());
    }

    fs::create_dir(repo_dir)?;
    fs::create_dir(&objects_dir)?;
    fs::write(&index_file, b"")?;

    fs::write(&head_file, b"ref: refs/heads/main\n")?;

    println!("Initialized empty rvcs repository in .rvcs/");
    Ok(())
}
