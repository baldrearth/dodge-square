use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

const PLAYER_SPEED: f32 = 500.0;
const ENEMY_SPEED: f32 = 200.0;

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
    commands.spawn((
        Enemy,
        Sprite::from_color(Color::srgb(1.0, 0.2, 0.2), Vec2::new(50.0, 50.0)),
        Transform::from_xyz(0.0, 250.0, 0.0),
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
){
    for mut transform in &mut query{        
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

fn move_enemy(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Enemy>>,
){
    for mut transform in &mut query{
        transform.translation.y -= ENEMY_SPEED * time.delta_secs();
    }
}

