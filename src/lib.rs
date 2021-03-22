// use std::ffi::OsStr;

// use bevy::{
//     asset::{AssetLoader, LoadedAsset},
//     prelude::*,
//     render::texture::{ImageType, Texture},
//     sprite::{Rect, TextureAtlas},
// };
// use spine::Atlas;

pub mod spine;
pub mod sprite;

// pub struct SpineImpoter;

// const EXTENSIONS: &'static [&'static str] = &["spine_json"];

// impl AssetLoader for SpineImpoter {
//     fn load<'a>(
//         &'a self,
//         bytes: &'a [u8],
//         load_context: &'a mut bevy::asset::LoadContext,
//     ) -> bevy::utils::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
//         Box::pin(async move {
//             let spine = spine::Spine::parse(bytes)?;

//             //let mut sprites: HashMap<String, usize> = Default::default();

//             // Load atlas with the same name
//             if let Ok(bytes) = load_context
//                 .read_asset_bytes(load_context.path().with_extension("spine_atlas"))
//                 .await
//             {
//                 let atlas = Atlas::parse(&bytes[..])?;

//                 // Load texture
//                 let texture_path = load_context.path().with_file_name(&atlas.name);
//                 let texture_extension = texture_path
//                     .extension()
//                     .map(OsStr::to_str)
//                     .flatten()
//                     .unwrap_or("");
//                 let texture_buffer = load_context.read_asset_bytes(&texture_path).await?;
//                 let texture = Texture::from_buffer(
//                     &texture_buffer[..],
//                     ImageType::Extension(texture_extension),
//                 )?;
//                 // TODO: Set texture format, filter and repeat attributes
//                 let texture =
//                     load_context.set_labeled_asset(&atlas.name, LoadedAsset::new(texture));

//                 // Crate texture atlas
//                 let mut texture_atlas = TextureAtlas::new_empty(texture, atlas.size.into());
//                 for region in &atlas.regions {
//                     let min: Vec2 = region.xy.into();
//                     let max: Vec2 = min + region.size.into();
//                     // TODO: Missing rotate and pivot attributes (likely to break frustum culling when added)
//                     texture_atlas.textures.push(Rect { min, max });
//                 }

//                 let texture_atlas =
//                     load_context.set_labeled_asset(&atlas.name, LoadedAsset::new(texture_atlas));
//             } else {
//                 // TODO: Fallback sprites from the spine `spine.skeleton.images`
//             }

//             let mut world = World::default();
//             let world_builder = &mut world.build();

//             Ok(())
//         })
//     }

//     fn extensions(&self) -> &[&str] {
//         EXTENSIONS
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
