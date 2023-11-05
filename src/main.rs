use bevy::prelude::*; 

#[derive(Component)]
struct Person; 

#[derive(Component)]
struct Name(String); 

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("P1".to_string())));
    commands.spawn((Person, Name("P2".to_string())));
    commands.spawn((Person, Name("P3".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}", name.0);
    }
}

fn hello_world() {
    println!("test"); 
}

fn main(){
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}
