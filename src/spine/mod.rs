//! Forked from crate `spine-data = "0.2.2"`
//!
//! Spine format documentation for (json)[http://pt.esotericsoftware.com/spine-json-format/]
//! and (binary)[http://pt.esotericsoftware.com/spine-binary-format]

use std::io::Read;

use anyhow::Error;
use atlas::Atlas;
use spine::Spine;

pub mod atlas;
pub mod spine;

pub struct SpineProject {
    pub spine: Spine,
    pub atlas: Atlas,
}

impl SpineProject {
    pub fn parse<S: Read, A: Read>(
        spine_reader: S,
        atlas_reader: A,
    ) -> Result<SpineProject, Error> {
        let spine = Spine::parse(spine_reader)?;
        let atlas = Atlas::parse(atlas_reader)?;

        Ok(SpineProject { spine, atlas })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;
    use anyhow::{Context, Error};
    use walkdir::WalkDir;

    #[test]
    fn test_load_spine_project() {
        for e in WalkDir::new("assets/").min_depth(1).max_depth(1) {
            if let Ok(e) = e {
                // Only directories
                if e.metadata().unwrap().is_file() {
                    continue;
                }

                let name = e.file_name().to_str().unwrap();

                let path = format!("assets/{0}/{0}.atlas", name);
                let _ = File::open(&path)
                    .map_err(Error::from)
                    .and_then(|file| Atlas::parse(file))
                    .with_context(|| format!("atlas file \"{}\"", &path))
                    .unwrap();

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
