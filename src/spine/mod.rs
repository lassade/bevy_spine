//! Forked from crate `spine-data = "0.2.2"`
//!
//! Spine format documentation for (json)[http://pt.esotericsoftware.com/spine-json-format/]
//! and (binary)[http://pt.esotericsoftware.com/spine-binary-format]

pub mod atlas;
pub mod spine;

pub use atlas::Atlas;
pub use spine::Spine;

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;
    use anyhow::{Context, Error};
    use walkdir::WalkDir;

    #[test]
    fn test_load_all_spine_examples() {
        for e in WalkDir::new("assets/").min_depth(1).max_depth(1) {
            if let Ok(e) = e {
                // Only directories
                if e.metadata().unwrap().is_file() {
                    continue;
                }

                let name = e.file_name().to_str().unwrap();

                // Not every spine file have an atlas
                let path = format!("assets/{0}/{0}.atlas", name);
                if let Ok(file) = File::open(&path) {
                    let _ = Atlas::parse(file)
                        .with_context(|| format!("atlas file \"{}\"", &path))
                        .unwrap();
                }

                for e in WalkDir::new(format!("assets/{}/", name)).max_depth(1) {
                    if let Ok(e) = e {
                        if e.metadata().unwrap().is_dir() {
                            continue;
                        }

                        if e.path().extension().map_or("", |os| os.to_str().unwrap()) == "json" {
                            let path = e.path().to_str().unwrap();
                            let _ = File::open(&path)
                                .map_err(Error::from)
                                .and_then(|file| Spine::parse(file))
                                .with_context(|| format!("spine file \"{}\"", &path))
                                .unwrap();
                        }
                    }
                }
            }
        }
    }
}
