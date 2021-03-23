use std::fs::File;

use bevy::prelude::*;
use bevy_spine::{
    spine::Atlas,
    sprite::{Rotation, Sprite, SpriteBundle, SpritePlugin, SpriteShape},
    transform::TransformMatrixPlugin,
};

#[derive(Default)]
struct AvailableSprites(Vec<Handle<Sprite>>);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpritePlugin)
        .add_plugin(TransformMatrixPlugin)
        .init_resource::<AvailableSprites>()
        .add_startup_system(setup.system())
        .add_system(update.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut available_sprites: ResMut<AvailableSprites>,
    mut sprites: ResMut<Assets<Sprite>>,
) {
    let reader = File::open("assets/hero/hero.atlas").unwrap();
    let atlas = Atlas::parse(reader).unwrap();

    let texture: Handle<Texture> = asset_server.load(format!("hero/{}", atlas.name).as_str());

    let atlas_sprites = &mut available_sprites.0;

    let mut atlas_size: Vec2 = atlas.size.into();
    atlas_size = atlas_size.recip();

    // ! FIXME: 1 pixel line and row is been trimmed
    for region in &atlas.regions {
        let size: Vec2 = region.size.into();
        let mut pivot: Vec2 = region.orig.into();
        let mut size_uv = size;

        if region.rotate {
            size_uv = Vec2::new(size_uv.y, size_uv.x);
            pivot = Vec2::new(pivot.y, pivot.x);
        }

        pivot *= size_uv.recip();
        size_uv *= atlas_size;

        let mut min: Vec2 = region.xy.into();
        min *= atlas_size;
        let mut max: Vec2 = min + size_uv;
        std::mem::swap(&mut min.y, &mut max.y);

        let mut sprite = Sprite::with_shape(
            Some(texture.clone()),
            SpriteShape::Rect {
                min,
                max,
                rotation: if region.rotate {
                    Rotation::CCW
                } else {
                    Rotation::None
                },
                size,
                pivot,
                padding: None,
            },
        );
        sprite.name = Some(region.name.clone());
        atlas_sprites.push(sprites.add(sprite));
    }

    commands.spawn(OrthographicCameraBundle::new_2d());

    commands.spawn(SpriteBundle {
        sprite: atlas_sprites[1].clone(),
        ..Default::default()
    });
}

#[derive(Default)]
struct Update {
    time_lapsed: f32,
}

fn update(
    mut lapsed: Local<Update>,
    time: Res<Time>,
    available_sprites: Res<AvailableSprites>,
    sprites: Res<Assets<Sprite>>,
    mut query: Query<&mut Handle<Sprite>>,
) {
    lapsed.time_lapsed += time.delta_seconds();

    let index = (lapsed.time_lapsed * 2.0).floor() as usize % available_sprites.0.len();
    let n = &available_sprites.0[index];

    for mut sprite in query.iter_mut() {
        // Display the sprite name when it changes
        if &*sprite != n {
            println!("{:?}", sprites.get(n).unwrap().name);
        }

        // Change the sprite
        *sprite = n.clone();
    }
}
