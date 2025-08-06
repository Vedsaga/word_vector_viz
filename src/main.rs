use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use std::collections::HashMap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 250.0,
            ..Default::default()
        })
        .init_resource::<AppState>()
        .init_resource::<UIState>()
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                draw_axes,
                handle_ui_interactions,
                update_vector_materials,
                update_vector_geometry,
                update_sphere_material,
                update_text_labels,
            )
                .chain(),
        )
        .add_systems(EguiPrimaryContextPass, ui_system)
        .run();
}

// === COMPONENTS ===

#[derive(Component)]
struct SphereMarker;

#[derive(Component)]
struct VectorTriangle {
    vector_id: String,
}

#[derive(Component)]
#[allow(dead_code)]
struct VectorEdge {
    vector_id: String,
    axis: VectorAxis,
}

#[derive(Component)]
#[allow(dead_code)]
struct VectorLabel {
    vector_id: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum VectorAxis {
    X,
    Y,
    Z,
}

// === RESOURCES ===

#[derive(Resource)]
struct AppState {
    sphere_transparency: f32,
    sphere_color: String,
    show_all_tags: bool,
    next_vector_id: u32,
    vectors: HashMap<String, VectorData>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            sphere_transparency: 0.3,
            sphere_color: "#808080".to_string(),
            show_all_tags: true,
            next_vector_id: 1,
            vectors: HashMap::new(),
        }
    }
}

#[derive(Resource, Default)]
struct UIState {
    panel_open: bool,
    expanded_vectors: std::collections::HashSet<String>,
}

#[derive(Clone)]
struct VectorData {
    internal_id: String,
    display_name: String,
    coordinates: [f32; 3],
    surface_visible: bool,
    edges_visible: bool,
    completely_visible: bool,
    tag_visible: bool,
    transparency: f32,
    color: String,
    triangle_entity: Option<Entity>,
    edge_entities: Vec<Entity>,
    label_entity: Option<Entity>,
}

impl VectorData {
    fn new(internal_id: String, display_name: String) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Generate random coordinates between -1 and 1
        let coordinates = [
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
        ];

        Self {
            internal_id,
            display_name,
            coordinates,
            surface_visible: true,
            edges_visible: true,
            completely_visible: true,
            tag_visible: true,
            transparency: 0.3,
            color: "#808080".to_string(),
            triangle_entity: None,
            edge_entities: Vec::new(),
            label_entity: None,
        }
    }
}

// === STARTUP SYSTEM ===

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // --- Sphere ---
    let _sphere_entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(0.5, 0.5, 0.5, 0.3),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            })),
            Transform::default(),
            SphereMarker,
        ))
        .id();

    // --- Directional Light ---
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // --- Camera ---
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(2.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
    ));
}

// === UPDATE SYSTEMS ===

fn draw_axes(mut gizmos: Gizmos) {
    // X-axis (Red)
    gizmos.line(Vec3::ZERO, Vec3::X, Color::srgb(1.0, 0.0, 0.0));
    // Y-axis (Green)
    gizmos.line(Vec3::ZERO, Vec3::Y, Color::srgb(0.0, 1.0, 0.0));
    // Z-axis (Blue)
    gizmos.line(Vec3::ZERO, Vec3::Z, Color::srgb(0.0, 0.0, 1.0));
}

fn ui_system(
    mut ui_state: ResMut<UIState>,
    mut app_state: ResMut<AppState>,
    mut contexts: EguiContexts,
) -> Result<()> {
    let ctx = contexts.ctx_mut()?;

    // Menu button in top-right
    egui::Window::new("Menu")
        .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::new(-10.0, 10.0))
        .resizable(false)
        .title_bar(false)
        .show(ctx, |ui| {
            if ui.button("☰").clicked() {
                ui_state.panel_open = !ui_state.panel_open;
            }
        });

    // Side panel
    if ui_state.panel_open {
        egui::SidePanel::right("vector_panel")
            .default_width(300.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Vector Controls");

                if ui.button("←").clicked() {
                    ui_state.panel_open = false;
                }

                ui.separator();

                // Global toggle
                ui.checkbox(&mut app_state.show_all_tags, "Show All Tags");

                ui.separator();

                // Sphere controls
                ui.heading("Sphere Settings");
                ui.add(
                    egui::Slider::new(&mut app_state.sphere_transparency, 0.0..=1.0)
                        .text("Transparency"),
                );
                ui.text_edit_singleline(&mut app_state.sphere_color);

                ui.separator();

                // Vector management
                ui.heading("Vector Management");

                if ui.button("Add Vector").clicked() {
                    let vector_id = format!("T{}", app_state.next_vector_id);
                    let display_name = format!("Untitled({})", app_state.next_vector_id);
                    let vector_data = VectorData::new(vector_id.clone(), display_name);
                    app_state.vectors.insert(vector_id.clone(), vector_data);
                    app_state.next_vector_id += 1;
                }

                ui.separator();

                // Vector list
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let vector_ids: Vec<String> = app_state.vectors.keys().cloned().collect();
                    for vector_id in vector_ids {
                        if let Some(vector_data) = app_state.vectors.get_mut(&vector_id) {
                            let is_expanded = ui_state.expanded_vectors.contains(&vector_id);

                            let response = ui.collapsing(
                                format!(
                                    "{}: {}",
                                    vector_data.internal_id, vector_data.display_name
                                ),
                                |ui| {
                                    // Name editing
                                    ui.horizontal(|ui| {
                                        ui.label("Name:");
                                        ui.text_edit_singleline(&mut vector_data.display_name);
                                    });

                                    // Coordinate sliders
                                    ui.horizontal(|ui| {
                                        ui.label("X:");
                                        ui.add(
                                            egui::Slider::new(
                                                &mut vector_data.coordinates[0],
                                                -1.0..=1.0,
                                            )
                                            .text("X")
                                            .fixed_decimals(2),
                                        );
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label("Y:");
                                        ui.add(
                                            egui::Slider::new(
                                                &mut vector_data.coordinates[1],
                                                -1.0..=1.0,
                                            )
                                            .text("Y")
                                            .fixed_decimals(2),
                                        );
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label("Z:");
                                        ui.add(
                                            egui::Slider::new(
                                                &mut vector_data.coordinates[2],
                                                -1.0..=1.0,
                                            )
                                            .text("Z")
                                            .fixed_decimals(2),
                                        );
                                    });

                                    // Visibility toggles
                                    ui.checkbox(
                                        &mut vector_data.surface_visible,
                                        "👁️ Triangle Surface",
                                    );
                                    ui.checkbox(&mut vector_data.edges_visible, "📏 Origin Edges");
                                    ui.checkbox(
                                        &mut vector_data.completely_visible,
                                        "Complete Visibility",
                                    );
                                    ui.checkbox(&mut vector_data.tag_visible, "🏷️ Show Label");

                                    // Appearance controls
                                    ui.add(
                                        egui::Slider::new(&mut vector_data.transparency, 0.0..=1.0)
                                            .text("Transparency"),
                                    );
                                    ui.horizontal(|ui| {
                                        ui.label("Color:");
                                        ui.text_edit_singleline(&mut vector_data.color);
                                    });

                                    // Delete button
                                    if ui.button("🗑️ Delete").clicked() {
                                        // Mark for deletion by setting completely_visible to false
                                        // The handle_ui_interactions system will clean up entities
                                        vector_data.completely_visible = false;
                                    }
                                },
                            );

                            // Track expanded state
                            if response.header_response.clicked() {
                                if is_expanded {
                                    ui_state.expanded_vectors.remove(&vector_id);
                                } else {
                                    ui_state.expanded_vectors.insert(vector_id);
                                }
                            }
                        }
                    }
                });
            });
    }
    Ok(())
}

fn handle_ui_interactions(
    mut commands: Commands,
    mut app_state: ResMut<AppState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _query_triangles: Query<Entity, With<VectorTriangle>>,
    _query_edges: Query<Entity, With<VectorEdge>>,
    _query_labels: Query<Entity, With<VectorLabel>>,
) {
    // Create new vector entities for vectors that don't have them yet
    let vectors_to_create: Vec<String> = app_state
        .vectors
        .iter()
        .filter(|(_, data)| data.triangle_entity.is_none() && data.completely_visible)
        .map(|(id, _)| id.clone())
        .collect();

    for vector_id in vectors_to_create {
        if let Some(vector_data) = app_state.vectors.get(&vector_id).cloned() {
            let triangle_entity =
                spawn_vector_triangle(&mut commands, &mut meshes, &mut materials, &vector_data);

            // Update the app state with the new entity
            if let Some(data) = app_state.vectors.get_mut(&vector_id) {
                data.triangle_entity = Some(triangle_entity);
            }
        }
    }

    // Handle vector deletion
    let vectors_to_delete: Vec<String> = app_state
        .vectors
        .iter()
        .filter(|(_, data)| !data.completely_visible && data.triangle_entity.is_some())
        .map(|(id, _)| id.clone())
        .collect();

    for vector_id in vectors_to_delete {
        if let Some(vector_data) = app_state.vectors.get(&vector_id) {
            // Despawn triangle entity
            if let Some(entity) = vector_data.triangle_entity {
                commands.entity(entity).despawn();
            }

            // Despawn edge entities
            for entity in &vector_data.edge_entities {
                commands.entity(*entity).despawn();
            }

            // Despawn label entity
            if let Some(entity) = vector_data.label_entity {
                commands.entity(entity).despawn();
            }
        }

        // Remove from app state
        app_state.vectors.remove(&vector_id);
    }
}

fn spawn_vector_triangle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    vector_data: &VectorData,
) -> Entity {
    let [x, y, z] = vector_data.coordinates;

    // Create triangle mesh
    let vertices = vec![
        Vec3::new(x, 0.0, 0.0), // X point
        Vec3::new(0.0, y, 0.0), // Y point
        Vec3::new(0.0, 0.0, z), // Z point
    ];

    // Calculate proper normals for the triangle
    let v1 = Vec3::new(x, 0.0, 0.0);
    let v2 = Vec3::new(0.0, y, 0.0);
    let v3 = Vec3::new(0.0, 0.0, z);

    let normal = (v2 - v1).cross(v3 - v1).normalize_or_zero();
    let normals = vec![normal; 3];

    let indices = vec![0u32, 1, 2];

    let mut triangle_mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::default(),
    );
    triangle_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    triangle_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    triangle_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![Vec2::ZERO; 3]);
    triangle_mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

    let color = parse_hex_color(&vector_data.color).unwrap_or(Color::srgba(
        0.5,
        0.5,
        0.5,
        vector_data.transparency,
    ));

    // Spawn triangle
    commands
        .spawn((
            Mesh3d(meshes.add(triangle_mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                alpha_mode: AlphaMode::Blend,
                unlit: false,
                ..default()
            })),
            Transform::default(),
            VectorTriangle {
                vector_id: vector_data.internal_id.clone(),
            },
        ))
        .id()
}

fn update_vector_materials(
    app_state: Res<AppState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        &VectorTriangle,
        &MeshMaterial3d<StandardMaterial>,
        &mut Visibility,
    )>,
) {
    if !app_state.is_changed() {
        return;
    }

    for (vector_triangle, material_handle, mut visibility) in query.iter_mut() {
        if let Some(vector_data) = app_state.vectors.get(&vector_triangle.vector_id) {
            // Update visibility
            *visibility = if vector_data.completely_visible && vector_data.surface_visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };

            // Update material if visible
            if let Some(material) = materials.get_mut(&material_handle.0) {
                let base_color =
                    parse_hex_color(&vector_data.color).unwrap_or(Color::srgb(0.5, 0.5, 0.5));

                material.base_color = Color::srgba(
                    base_color.to_srgba().red,
                    base_color.to_srgba().green,
                    base_color.to_srgba().blue,
                    vector_data.transparency,
                );
            }
        }
    }
}

fn update_vector_geometry(
    app_state: Res<AppState>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(&VectorTriangle, &Mesh3d)>,
) {
    if !app_state.is_changed() {
        return;
    }

    for (vector_triangle, mesh_handle) in query.iter() {
        if let Some(vector_data) = app_state.vectors.get(&vector_triangle.vector_id) {
            if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
                let [x, y, z] = vector_data.coordinates;
                let vertices = vec![
                    Vec3::new(x, 0.0, 0.0),
                    Vec3::new(0.0, y, 0.0),
                    Vec3::new(0.0, 0.0, z),
                ];

                // Calculate proper normals
                let v1 = Vec3::new(x, 0.0, 0.0);
                let v2 = Vec3::new(0.0, y, 0.0);
                let v3 = Vec3::new(0.0, 0.0, z);
                let normal = (v2 - v1).cross(v3 - v1).normalize_or_zero();
                let normals = vec![normal; 3];

                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            }
        }
    }
}

fn update_sphere_material(
    app_state: Res<AppState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&MeshMaterial3d<StandardMaterial>, With<SphereMarker>>,
) {
    if !app_state.is_changed() {
        return;
    }

    for material_handle in query.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            let base_color =
                parse_hex_color(&app_state.sphere_color).unwrap_or(Color::srgb(0.5, 0.5, 0.5));

            material.base_color = Color::srgba(
                base_color.to_srgba().red,
                base_color.to_srgba().green,
                base_color.to_srgba().blue,
                app_state.sphere_transparency,
            );
        }
    }
}

fn update_text_labels(app_state: Res<AppState>, mut gizmos: Gizmos) {
    if !app_state.show_all_tags {
        return;
    }

    for (_vector_id, vector_data) in &app_state.vectors {
        if !vector_data.completely_visible || !vector_data.tag_visible {
            continue;
        }

        let [x, y, z] = vector_data.coordinates;

        // Draw labels at midpoints of edges
        if vector_data.edges_visible {
            let _label = format!("{}: {}", vector_data.internal_id, vector_data.display_name);

            // Position at average of the triangle vertices
            let label_pos = Vec3::new(x / 3.0, y / 3.0, z / 3.0);

            // Draw a small sphere at label position as placeholder for text
            gizmos.sphere(label_pos, 0.05, Color::srgb(1.0, 1.0, 1.0));
        }
    }

    // Draw vector edges using gizmos
    for (_vector_id, vector_data) in &app_state.vectors {
        if !vector_data.completely_visible || !vector_data.edges_visible {
            continue;
        }

        let [x, y, z] = vector_data.coordinates;

        // Origin edges
        gizmos.line(
            Vec3::ZERO,
            Vec3::new(x, 0.0, 0.0),
            Color::srgb(1.0, 0.0, 0.0),
        ); // Red X
        gizmos.line(
            Vec3::ZERO,
            Vec3::new(0.0, y, 0.0),
            Color::srgb(0.0, 1.0, 0.0),
        ); // Green Y
        gizmos.line(
            Vec3::ZERO,
            Vec3::new(0.0, 0.0, z),
            Color::srgb(0.0, 0.0, 1.0),
        ); // Blue Z
    }
}

// === UTILITY FUNCTIONS ===

fn parse_hex_color(hex: &str) -> Option<Color> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return None;
    }

    let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
    let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
    let b = u8::from_str_radix(&hex[5..7], 16).ok()?;

    Some(Color::srgb(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
    ))
}
