use bevy::{prelude::*, ecs::query, transform};


#[derive(Component)]
struct Person;


#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer); 

fn add_people(mut commands: Commands){
    commands.spawn((Person, Name("x".to_string())));
    commands.spawn((Person, Name("y".to_string())));
}
/*
fn hello_world(){
    println!("Hello, World!");
}
*/ 

fn greet_people(time: Res<Time>, mut timer : ResMut <GreetTimer>, query : Query<&Name, With<Person>>){
    if timer.0.tick(time.delta()).just_finished(){
        for name in query.iter(){
            println!("hello {}!", name.0);
        }     
    }
    
  
}
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App){
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
         //   .add_system(hello_world)
            .add_system(greet_people);
    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run(); 
}
