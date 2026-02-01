use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub struct Index {
    pub entries: BTreeMap<String, String>,
}

impl Index {
    pub fn load(index_path: &Path) -> anyhow::Result<Self> {
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

    pub fn save(&self, index_path: &Path) -> anyhow::Result<()> {
        let out: String = self
            .entries
            .iter()
            .map(|(path, hash)| format!("{hash} {path}\n"))
            .collect();

        fs::write(index_path, out)?;
        Ok(())
    }

    pub fn add(&mut self, path: String, hash: String) {
        self.entries.insert(path, hash);
    }
}
