use bevy::prelude::*;
use rand::prelude::random;
use bevy::core::FixedTimestep;

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
        .add_startup_system(spawn_wall)
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
                .with_system(snake_eating.after(move_snake)),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_system(game_over.after(move_snake))
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
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
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
struct Food;

#[derive(Component)]
struct Wall;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

const ENEMY_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

const AREA_WIDTH: u32 = 500;
const AREA_HEIGHT: u32 = 500;

const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

fn spawn_enemy(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: ENEMY_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 20.0),
                ..default()
            },
            ..default()
        })
        .insert(Enemy)
        .insert(Position { x: 5, y: 5 })
        .insert(Size::square(0.8));
}

fn spawn_wall(mut commands: Commands) {
    for n in 0..11 {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                ..default()
            })
            .insert(Wall)
            .insert(Position { x: -1, y: n-1 })
            .insert(Size::square(0.8));
        
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                ..default()
            })
            .insert(Wall)
            .insert(Position { x: 10, y: n })
            .insert(Size::square(0.8));
            
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                ..default()
            })
            .insert(Wall)
            .insert(Position { x: n, y: -1 })
            .insert(Size::square(0.8));
            
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                ..default()
            })
            .insert(Wall)
            .insert(Position { x: n-1, y: 10 })
            .insert(Size::square(0.8));
    }
}

fn food_spawner(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}

fn move_snake(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Position, With<SnakeHead>>,
) {
    for mut pos in query.iter_mut() {
        
        if keyboard_input.pressed(KeyCode::Left) {
            pos.x -= 1;
        }
    
        if keyboard_input.pressed(KeyCode::Right) {
            pos.x += 1;
        }
    
        if keyboard_input.pressed(KeyCode::Up) {
            pos.y += 1;
        }
    
        if keyboard_input.pressed(KeyCode::Down) {
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

fn position_translation(mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, AREA_WIDTH as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, AREA_HEIGHT as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn size_scaling(mut q: Query<(&Size, &mut Transform)>) {
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * AREA_WIDTH as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * AREA_HEIGHT as f32,
            1.0,
        );
    }
}

fn snake_eating(
    mut commands: Commands,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
    enemy_positions: Query<&Position, With<Enemy>>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
            }
        }
    }
    for head_pos in head_positions.iter() {
        for enemy_pos in enemy_positions.iter() {
            if enemy_pos == head_pos {
                game_over_writer.send(GameOverEvent);
            }
        }
    }
    for enemy_pos in enemy_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == enemy_pos {
                commands.entity(ent).despawn();
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
