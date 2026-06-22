use bevy::prelude::*;
use rand::random_range;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Resource)]
struct GameOver{
    is_over: bool,
}


const PLAYER_SPEED: f32 = 500.0;
const ENEMY_SPEED: f32 = 500.0;

const PLAYER_X_MIN: f32 = -350.0;
const PLAYER_X_MAX: f32 = 350.0;


const ENEMY_SPAWN_Y_MIN: f32 = 350.0;
const ENEMY_SPAWN_Y_MAX: f32 = 700.0;
const ENEMY_RESET_Y: f32 = -350.0;
const ENEMY_X_MIN: f32 = -350.0;
const ENEMY_X_MAX: f32 = 350.0;

const ENEMY_COUNT: usize = 3;

const PLAYER_SIZE: f32 = 60.0;
const ENEMY_SIZE: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameOver {is_over: false})
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, move_enemy, check_collision))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player,
        Sprite::from_color(Color::srgb(0.3, 0.7, 1.0), Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
        Transform::from_xyz(0.0, -250.0, 0.0),
    ));
    for _ in 0..ENEMY_COUNT {
        commands.spawn((
            Enemy,
            Sprite::from_color(Color::srgb(1.0, 0.2, 0.2), Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
            Transform::from_xyz(
                random_range(ENEMY_X_MIN..ENEMY_X_MAX),
                random_range(ENEMY_SPAWN_Y_MIN..ENEMY_SPAWN_Y_MAX),
                0.0,
            ),
        ));
    }
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game_over: Res<GameOver>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if game_over.is_over{
        return;
    }
    for mut transform in &mut query {
        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += 1.0;
        }

        transform.translation.x += direction * PLAYER_SPEED * time.delta_secs();
        transform.translation.x = transform.translation.x.clamp(PLAYER_X_MIN, PLAYER_X_MAX);
    }
}

fn move_enemy(
    time: Res<Time>, 
    game_over: Res<GameOver>,
    mut query: Query<&mut Transform, With<Enemy>>) {
    for mut transform in &mut query {
        if game_over.is_over{
            return;
        }
        transform.translation.y -= ENEMY_SPEED * time.delta_secs();
        if transform.translation.y < ENEMY_RESET_Y {
            transform.translation.y = random_range(ENEMY_SPAWN_Y_MIN..ENEMY_SPAWN_Y_MAX);
            transform.translation.x = random_range(ENEMY_X_MIN..ENEMY_X_MAX);
        }
    }
}

fn spawn_game_over_text(commands: &mut Commands) {
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_child((
            Text::new("GAME OVER"),
            TextFont {
                font_size: 80.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 1.0)),
        ));
}

fn check_collision(
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut game_over: ResMut<GameOver>,
    mut commands: Commands,
) {
    if game_over.is_over{
        return;
    }
    for player_transform in &player_query {
        for enemy_transform in &enemy_query {
            let player_pos = player_transform.translation;
            let enemy_pos = enemy_transform.translation;

            let x_distance = (player_pos.x - enemy_pos.x).abs();
            let y_distance = (player_pos.y - enemy_pos.y).abs();

            let x_collision = x_distance < (PLAYER_SIZE + ENEMY_SIZE) / 2.0;
            let y_collision = y_distance < (PLAYER_SIZE + ENEMY_SIZE) / 2.0;

            if x_collision && y_collision {
                println!("Game Over");
                game_over.is_over = true;
                spawn_game_over_text(&mut commands);
            }
        }
    }
}
