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
        app.add_startup_system(setup);
    }
}

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
    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(get_sprite_index_from_coords(21, 0)),
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE)),
        ..default()
    });
}

fn get_sprite_index_from_coords(x: usize, y: usize) -> usize {
    return x * SPRITE_COLS + y;
}
