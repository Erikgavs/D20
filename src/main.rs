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
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Create the icosahedron (d20) - a sphere subdivided into an icosahedron with 0 subdivisions
    // Sphere::new(1.0) creates a sphere with radius 1.0
    // .mesh().ico(0) converts it to an icosahedron with 0 subdivisions (20 triangular faces)
    // .unwrap() extracts the mesh from the result
    let d20_mesh = Sphere::new(1.0).mesh().ico(0).unwrap();
    
    // Spawn the dice entity with mesh and material
    commands.spawn((
        Mesh3d(meshes.add(d20_mesh)),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))), // Golden color for the dice
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
    
    // Add camera to see the scene
    // Position the camera at (0, 0, 5) looking at the center of the world (0, 0, 0)
    // Vec3::new(0.0, 0.0, 5.0) = camera position (5 units away on Z axis)
    // Vec3::ZERO is shorthand for Vec3::new(0.0, 0.0, 0.0) = look target (dice position)
    // Vec3::Y means the positive Y axis (0, 1, 0) is considered "up" orientation
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)) // puts the camera 5 units distance from the dice on Z axis
            .looking_at(Vec3::ZERO, Vec3::Y), // look at center where dice is, Y axis as "up"
    ));
    
    // Add directional light to illuminate the dice properly
    commands.spawn(DirectionalLight::default()); // default direction light coming from above
}