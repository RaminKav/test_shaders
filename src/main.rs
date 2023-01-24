//! A shader and a material that uses it.

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<CustomMaterial>::default())
        .add_startup_system(setup)
        .add_system(mouse_click_system)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // cube
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),

        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(CustomMaterial {
            // color: Color::BLUE,
            source_texture: Some(asset_server.load("uv_map_int.png")),
            lookup_texture: Some(asset_server.load("palette2.png")),
            // alpha_mode: AlphaMode::Blend,
        }),
        ..default()
    });

    // camera
    commands.spawn(Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..Default::default()
        },
        ..Default::default()
    });
}
fn mouse_click_system(
    key_input: ResMut<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    query: Query<&Handle<CustomMaterial>>,
) {
    // if key_input.pressed(KeyCode::A) {
    //     println!("clicked");
    //     let x = materials.get_mut(query.get_single().unwrap());
    //     if let Some(mat) = x {
    //         println!("AYY");
    //         mat.source_texture = Some(asset_server.load("textures/source_2.png"))
    //     }
    // }
    // if key_input.pressed(KeyCode::S) {
    //     let x = materials.get_mut(query.single());
    //     if let Some(mat) = x {
    //         println!("AYY2");

    //         mat.source_texture = Some(asset_server.load("textures/source_1.png"))
    //     }
    // }
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    // fn alpha_mode(&self) -> AlphaMode {
    //     self.alpha_mode
    // }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "9600d1e3-1911-4286-9810-e9bd9ff685e1"]
pub struct CustomMaterial {
    #[texture(0)]
    #[sampler(1)]
    source_texture: Option<Handle<Image>>,
    #[texture(2)]
    #[sampler(3)]
    lookup_texture: Option<Handle<Image>>,
}
