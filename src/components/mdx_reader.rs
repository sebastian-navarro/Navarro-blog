use std::fs;
use std::path::Path;

pub fn read_mdx_files(directory: &str) -> Result<Vec<String>, std::io::Error> {
    let mut posts = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "mdx" {
            let content = fs::read_to_string(&path)?;
            posts.push(content);
        }
    }
    Ok(posts)
}

