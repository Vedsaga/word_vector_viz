use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        // The example confirms `DefaultPlugins` is sufficient. No other plugins are needed.
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 250.0,
            ..Default::default()
        })
        .add_systems(Startup, setup_scene)
        .add_systems(Update, draw_axes)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // --- Sphere ---
    // This is the new, correct pattern from the official 0.16 example.
    // We spawn a tuple of special components: `Mesh3d` and `MeshMaterial3d`.
    commands.spawn((
        // The Mesh3d component, which takes a handle to our mesh.
        Mesh3d(meshes.add(Sphere::new(1.0))),
        // The MeshMaterial3d component, which takes a handle to our material.
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.8, 0.8, 0.8, 0.3),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        })),
        // The standard Transform component.
        Transform::default(),
    ));

    // --- Directional Light ---
    // The example confirms this component-based pattern is correct.
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // --- Camera ---
    // The example also shows a simplified way to spawn the 3D camera.
    commands.spawn((
        // Spawning `Camera3d::default()` seems to be the new bundle.
        Camera3d::default(),
        Transform::from_xyz(2.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        // We still add our pan-orbit component to the same entity.
        PanOrbitCamera::default(),
    ));
}

/// A system that draws the coordinate axes using Gizmos. This was already correct.
fn draw_axes(mut gizmos: Gizmos) {
    // X-axis (Red)
    gizmos.line(Vec3::ZERO, Vec3::X, Color::srgb(1.0, 0.0, 0.0));
    // Y-axis (Green)
    gizmos.line(Vec3::ZERO, Vec3::Y, Color::srgb(0.0, 1.0, 0.0));
    // Z-axis (Blue)
    gizmos.line(Vec3::ZERO, Vec3::Z, Color::srgb(0.0, 0.0, 1.0));
}
