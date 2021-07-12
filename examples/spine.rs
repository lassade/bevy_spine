use std::fs::File;

use bevy::prelude::*;
use bevy_spine::{
    spine::Atlas,
    sprite::{Rotation, Sprite, SpriteBundle, SpritePlugin, SpriteShape},
    SpinePlugin,
};

#[derive(Default)]
struct AvailableSprites(Vec<Handle<Sprite>>);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpritePlugin)
        .add_plugin(SpinePlugin)
        .init_resource::<AvailableSprites>()
        .add_startup_system(setup.system())
        //.add_system(update.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut available_sprites: ResMut<AvailableSprites>,
    mut sprites: ResMut<Assets<Sprite>>,
) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
    // commands.spawn(SpriteBundle {
    //     sprite: atlas_sprites[1].clone(),
    //     ..Default::default()
    // });
}
