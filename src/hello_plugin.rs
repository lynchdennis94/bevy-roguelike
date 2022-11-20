use bevy::prelude::*;

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Once)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Alice Apple".to_string())));
    commands.spawn((Person, Name("Bobby Burgers".to_string())));
    commands.spawn((Person, Name("Candace Canopy".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello {}!", name.0);
        }
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);
