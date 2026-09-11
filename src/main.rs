use bevy::prelude::*;

fn main() {
    // Application startup
    App::new()
        .add_plugins(DefaultPlugins) // gets all the basic features of bevy with this line
        .add_systems(Startup, setup)
        .run();
}

// Bevy will call this once when we run the code.
// Startup runs once: here we build the initial scene (camera, light, later the dice).
//
// mut meshes: ResMut<Assets<Mesh>> -> shape box: stores the d20's geometry
//
// ResMut -> mutable access to the resource: lets us save our d20 dice (geometrical form)
//
// mut materials: ResMut<Assets<StandardMaterial>> -> paint box: stores the materials to fill/paint the shapes
//
// commands are essential: the spawned entities (camera, light) exist for the whole app
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(Camera3d::default()); // adds a camera to the scene. Just to be able to see the dice
    commands.spawn(DirectionalLight::default());
}
