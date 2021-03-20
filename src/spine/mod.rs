//! Forked from crate `spine-data = "0.2.2"`

use std::io::Read;

use atlas::Atlas;
use failure::Error;
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
    use super::*;
    use std::fs::File;

    #[test]
    fn test_load_spine_project() {
        let json = File::open("Cowbot/Cowbot.json").expect("Could not open JSON file");
        let atlas = File::open("Cowbot/Cowbot_tex.atlas").expect("Could not open Atlas file");
        SpineProject::parse(json, atlas).unwrap();
    }
}
