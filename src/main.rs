use bevy::{
    prelude::*,
    time::FixedTimestep,
    sprite::collide_aabb::{collide},
};
use rand::prelude::random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_startup_system(spawn_enemy)
        .add_startup_system(spawn_scoreboard)
        .add_event::<GameOverEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food_spawner),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(move_snake)
                .with_system(check_for_collisions.after(move_snake)),
        )
        .add_system(game_over.after(move_snake))
        .add_system(update_scoreboard)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

struct GameOverEvent;

struct Scoreboard {
    score: usize,
}

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct Enemy;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Food;

#[derive(Component)]
struct Collider;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

const ENEMY_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

const TIME_STEP: f32 = 1.0 / 30.0;
const PADDLE_SPEED: f32 = 600.0;

const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 200.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 30.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead);
}

fn spawn_enemy(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: ENEMY_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 100.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 30.0),
                ..default()
            },
            ..default()
        })
        .insert(Enemy)
        .insert(Collider);
}

fn spawn_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        }),
    );
}

fn food_spawner(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(random::<f32>() * 300.0, random::<f32>() * 300.0, 0.0),
                scale: Vec3::new(20.0, 20.0, 20.0),
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Collider);
}

fn move_snake(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<SnakeHead>>,
) {
    let mut paddle_transform = query.single_mut();
    let mut xdirection = 0.0;
    let mut ydirection = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        xdirection -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        xdirection += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        ydirection += 1.0;
    }
    
    if keyboard_input.pressed(KeyCode::Down) {
        ydirection -= 1.0;
    }

    let new_paddle_xposition = paddle_transform.translation.x + xdirection * PADDLE_SPEED * TIME_STEP;
    let new_paddle_yposition = paddle_transform.translation.y + ydirection * PADDLE_SPEED * TIME_STEP;

    paddle_transform.translation.x = new_paddle_xposition;
    paddle_transform.translation.y = new_paddle_yposition;

}

fn check_for_collisions(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut head_positions: Query<&Transform, With<SnakeHead>>,
    collider_query: Query<(Entity, &Transform, Option<&Food>, Option<&Enemy>), With<Collider>>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    let head_transform = head_positions.single_mut();
    let head_size = head_transform.scale.truncate();

    for (collider_entity, transform, maybe_food, maybe_enemy) in &collider_query {
        let collision = collide(
            head_transform.translation,
            head_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {

            if maybe_food.is_some() {
                scoreboard.score += 1;
                commands.entity(collider_entity).despawn();
            }

            if maybe_enemy.is_some() {
                game_over_writer.send(GameOverEvent);
            }
        }
    }

}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    food: Query<Entity, With<Food>>,
    head: Query<Entity, With<SnakeHead>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter() {
            commands.entity(ent).despawn();
        }
        for ent in head.iter() {
            commands.entity(ent).despawn();
        } 
    }
}
