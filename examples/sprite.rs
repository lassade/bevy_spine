use std::fs::File;

use bevy::prelude::*;
use bevy_spine::{
    spine::Atlas,
    sprite::{Rotation, Sprite, SpriteBundle, /*SpriteInstance,*/ SpritePlugin, SpriteShape},
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpritePlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    //mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sprites: ResMut<Assets<Sprite>>,
) {
    let reader = File::open("assets/hero/hero.atlas").unwrap();
    let atlas = Atlas::parse(reader).unwrap();

    let texture: Handle<Texture> = asset_server.load(format!("hero/{}", atlas.name).as_str());

    let mut atlas_sprites = vec![];
    for region in &atlas.regions {
        let size = region.size.into();
        let min: Vec2 = region.xy.into();
        let max: Vec2 = min + region.size.into();
        // dbg!(region);
        atlas_sprites.push(sprites.add(Sprite::with_shape(
            Some(texture.clone()),
            SpriteShape::Rect {
                min,
                max,
                rotation: if region.rotate {
                    Rotation::CW
                } else {
                    Rotation::None
                },
                size,
                pivot: Vec2::splat(0.5),
                padding: None,
            },
        )));
    }

    commands.spawn(OrthographicCameraBundle::new_2d());

    //meshes.set_untracked(Handle::<Mesh>::default(), Mesh::default());

    commands.spawn(SpriteBundle {
        sprite: atlas_sprites[1].clone(),
        ..Default::default()
    });

    // commands.spawn(PbrBundle {
    //     mesh: Handle::<Mesh>::default(), //meshes.add(shape::Quad::new(Vec2::splat(100.0)).into()),
    //     ..Default::default()
    // });

    // commands
    //     .spawn(PbrBundle {
    //         //mesh: meshes.add(shape::Quad::new(Vec2::one()).into()),
    //         ..Default::default()
    //     })
    //     .with(atlas_sprites[1].clone());

    // commands
    //     .spawn(SpriteBundle {
    //         material: materials.add(texture_handle.into()),
    //         ..Default::default()
    //     });
}
