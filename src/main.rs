use bevy::prelude::*;
use rand::random_range;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

const PLAYER_SPEED: f32 = 500.0;
const ENEMY_SPEED: f32 = 200.0;

const ENEMY_SPAWN_Y_MIN: f32 = 350.0;
const ENEMY_SPAWN_Y_MAX: f32 = 700.0;
const ENEMY_RESET_Y: f32 = -350.0;
const ENEMY_X_MIN: f32 = -350.0;
const ENEMY_X_MAX: f32 = 350.0;

const ENEMY_COUNT: usize = 3;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, move_enemy))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player,
        Sprite::from_color(Color::srgb(0.3, 0.7, 1.0), Vec2::new(60.0, 60.0)),
        Transform::from_xyz(0.0, -250.0, 0.0),
    ));
    for _ in 0..ENEMY_COUNT {
        commands.spawn((
            Enemy,
            Sprite::from_color(Color::srgb(1.0, 0.2, 0.2), Vec2::new(50.0, 50.0)),
            Transform::from_xyz(
                random_range(ENEMY_X_MIN..ENEMY_X_MAX),
                random_range(ENEMY_SPAWN_Y_MIN..ENEMY_SPAWN_Y_MAX), 
                0.0
            ),
        ));
    }
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in &mut query {
        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += 1.0;
        }

        transform.translation.x += direction * PLAYER_SPEED * time.delta_secs();
    }
}

fn move_enemy(time: Res<Time>, mut query: Query<&mut Transform, With<Enemy>>) {
    for mut transform in &mut query {
        transform.translation.y -= ENEMY_SPEED * time.delta_secs();
        if transform.translation.y < ENEMY_RESET_Y {
            transform.translation.y = random_range(ENEMY_SPAWN_Y_MIN..ENEMY_SPAWN_Y_MAX);
            transform.translation.x = random_range(ENEMY_X_MIN..ENEMY_X_MAX);
        }
    }
}
