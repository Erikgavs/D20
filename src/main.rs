use bevy::prelude::*;

fn main() {
    // Aplication Startup
    App::new()
        .add_plugins(DefaultPlugins) // Getting all the basic features of beavy w this line
        .add_systems(Startup, setup)
        .run();
}

// bevy will call this once when we run the code
fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default()); // adds a camera to the scene. Just to be able to see the dice
}
