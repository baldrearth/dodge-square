use bevy::prelude::*;

#[derive(Component)]
struct Player;

const PLAYER_SPEED: f32 = 500.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player,
        Sprite::from_color(Color::srgb(0.3, 0.7, 1.0), Vec2::new(60.0, 60.0)),
        Transform::from_xyz(0.0, -250.0, 0.0),
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

