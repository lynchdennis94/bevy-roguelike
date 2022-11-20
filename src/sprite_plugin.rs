use bevy::prelude::*;

const ASSET_FILE_PATH: &str = "urizen_combined_basic_and_medieval.png";
const SPRITE_TILE_X: f32 = 13.0;
const SPRITE_TILE_Y: f32 = 13.0;
const SPRITE_PADDING_X: f32 = 0.0;
const SPRITE_PADDING_Y: f32 = 0.0;
const SPRITE_OFFSET_X: f32 = 1.0;
const SPRITE_OFFSET_Y: f32 = 1.0;
const SPRITE_ROWS: usize = 30;
const SPRITE_COLS: usize = 20;
const SPRITE_SCALE: f32 = 3.0;

pub struct SpritePlugin;
impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(move_sprite);
    }
}

#[derive(Component)]
struct Movable;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load in the atlas
    let texture_handle: Handle<Image> = asset_server.load(ASSET_FILE_PATH);
    let texture_atlas: TextureAtlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(SPRITE_TILE_X, SPRITE_TILE_Y),
        SPRITE_COLS,
        SPRITE_ROWS,
        Some(Vec2::new(SPRITE_PADDING_X, SPRITE_PADDING_Y)),
        Some(Vec2::new(SPRITE_OFFSET_X, SPRITE_OFFSET_Y)),
    );
    let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

    // Spawn in a camera
    commands.spawn(Camera2dBundle::default());

    // Spawn in a sprite from the atlas
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(get_sprite_index_from_coords(21, 0)),
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE)),
            ..default()
        },
        Movable,
    ));
}

fn get_sprite_index_from_coords(x: usize, y: usize) -> usize {
    return x * SPRITE_COLS + y;
}

fn apply_movement_units_to_translation(mut translation: Vec3, x: f32, y: f32) -> Vec3 {
    translation.x += x * SPRITE_TILE_X * SPRITE_SCALE;
    translation.y += y * SPRITE_TILE_Y * SPRITE_SCALE;
    return translation;
}

fn move_sprite(
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&Movable, &mut Transform)>,
) {
    for (_, mut transform) in &mut sprite_position {
        if keyboard_input.just_pressed(KeyCode::Numpad4) {
            transform.translation =
                apply_movement_units_to_translation(transform.translation, -1.0, 0.0);
        } else if keyboard_input.just_pressed(KeyCode::Numpad6) {
            transform.translation =
                apply_movement_units_to_translation(transform.translation, 1.0, 0.0);
        } else if keyboard_input.just_pressed(KeyCode::Numpad8) {
            transform.translation =
                apply_movement_units_to_translation(transform.translation, 0.0, 1.0);
        } else if keyboard_input.just_pressed(KeyCode::Numpad2) {
            transform.translation =
                apply_movement_units_to_translation(transform.translation, 0.0, -1.0);
        } else if keyboard_input.just_pressed(KeyCode::Numpad7) {
            transform.translation =
                apply_movement_units_to_translation(transform.translation, -1.0, 1.0);
        } else if keyboard_input.just_pressed(KeyCode::Numpad9) {
            transform.translation =
                apply_movement_units_to_translation(transform.translation, 1.0, 1.0);
        } else if keyboard_input.just_pressed(KeyCode::Numpad3) {
            transform.translation =
                apply_movement_units_to_translation(transform.translation, 1.0, -1.0);
        } else if keyboard_input.just_pressed(KeyCode::Numpad1) {
            transform.translation =
                apply_movement_units_to_translation(transform.translation, -1.0, -1.0);
        }
    }
}
