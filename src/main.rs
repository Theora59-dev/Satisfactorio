use bevy::prelude::*;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, 
        add_people
    )
    .add_systems(Update, 
        greet_people
    )
    .run();

}

#[derive(Component)]
struct Person;




#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}


fn hello_world(time: Res<Time>) {
    println!("Hello, Bevy!");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}
