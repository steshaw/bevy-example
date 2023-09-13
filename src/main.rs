use bevy::prelude::*;

#[derive(Component)]
struct Position { x: f32, y: f32 }

fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}

struct Alien(u64);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn startup_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn hello_world() {
    println!("hello world!");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

pub struct OurPlugin;

impl Plugin for OurPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup_people)
            .add_systems(Update, (greet_people, hello_world))
            .add_systems(Update, print_position_system);
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(OurPlugin)
        .run();
}
