use std::path::{Component, Path, PathBuf};

pub fn sanitize_path(path: &Path) -> PathBuf {
    let mut clean = PathBuf::new();
    
    for component in path.components() {
        match component {
            Component::ParentDir => {
                clean.pop();
            },
            Component::Normal(c) => {
                clean.push(c);
            },
            Component::RootDir => {
                clean.push("/");
            },
            _ => {}
        }
    }

    clean
}