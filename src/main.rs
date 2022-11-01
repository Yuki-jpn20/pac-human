use bevy::{
    prelude::*,
    time::FixedTimestep,
    sprite::collide_aabb::{collide},
};
use rand::prelude::random;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake".to_string(),
            width: 600.0,
            height: 600.0,
            ..default()
        })
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_startup_system(spawn_enemy)
        //.add_startup_system(spawn_wall)
        .add_event::<GameOverEvent>()
        //.add_system(snake_movement_input.before(move_snake))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food_spawner),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(move_snake)
                .with_system(move_enemy)
                .with_system(check_for_collisions.after(move_snake)),
        )
        .add_system(game_over.after(move_snake))
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

struct GameOverEvent;

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
struct Wall;

#[derive(Component)]
struct Collider;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

const ENEMY_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

const TIME_STEP: f32 = 1.0 / 60.0;
const PADDLE_SPEED: f32 = 500.0;

// const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

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

// fn spawn_wall(mut commands: Commands) {
//     for n in 0..11 {
//         commands
//             .spawn_bundle(SpriteBundle {
//                 sprite: Sprite {
//                     color: WALL_COLOR,
//                     ..default()
//                 },
//                 transform: Transform {
//                     scale: Vec3::new(10.0, 10.0, 10.0),
//                     ..default()
//                 },
//                 ..default()
//             })
//             .insert(Wall);
        
//         commands
//             .spawn_bundle(SpriteBundle {
//                 sprite: Sprite {
//                     color: WALL_COLOR,
//                     ..default()
//                 },
//                 transform: Transform {
//                     scale: Vec3::new(10.0, 10.0, 10.0),
//                     ..default()
//                 },
//                 ..default()
//             })
//             .insert(Wall);
            
//         commands
//             .spawn_bundle(SpriteBundle {
//                 sprite: Sprite {
//                     color: WALL_COLOR,
//                     ..default()
//                 },
//                 transform: Transform {
//                     scale: Vec3::new(10.0, 10.0, 10.0),
//                     ..default()
//                 },
//                 ..default()
//             })
//             .insert(Wall);
            
//         commands
//             .spawn_bundle(SpriteBundle {
//                 sprite: Sprite {
//                     color: WALL_COLOR,
//                     ..default()
//                 },
//                 transform: Transform {
//                     scale: Vec3::new(10.0, 10.0, 10.0),
//                     ..default()
//                 },
//                 ..default()
//             })
//             .insert(Wall);
//     }
// }

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

fn move_enemy(
    mut query: Query<&mut Position, With<Enemy>>,
) {
    let dir = (random::<f32>() * 4.) as i32;
    for mut pos in query.iter_mut() {
        if dir == 0 {
            pos.x -= 1;
        }

        if dir == 1 {
            pos.x += 1;
        }

        if dir == 2 {
            pos.y += 1;
        }

        if dir == 3 {
            pos.y -= 1;
        }
    }

    for mut pos in query.iter_mut() {
        if pos.x < 0 {
            pos.x += 1;
        }

        if pos.x as u32 >= ARENA_WIDTH {
            pos.x -= 1;
        }

        if pos.y < 0 {
            pos.y += 1;
        }

        if pos.y as u32 >= ARENA_HEIGHT {
            pos.y -= 1;
        }
    }
}

fn check_for_collisions(
    mut commands: Commands,
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
                commands.entity(collider_entity).despawn();
            }

            if maybe_enemy.is_some() {
                game_over_writer.send(GameOverEvent);
            }
        }
    }

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
