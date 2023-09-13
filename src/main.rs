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

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello name={} time={:?}!", name.0, time.elapsed());
        }
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct OurPlugin;

impl Plugin for OurPlugin {
    fn build(&self, app: &mut App) {
        let timer = GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating));
        app
            .insert_resource(timer)
            .add_systems(Startup, startup_people)
            .add_systems(Update, greet_people)
            .add_systems(Update, print_position_system);
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(OurPlugin)
        .run();
}
