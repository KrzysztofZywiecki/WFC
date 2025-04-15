use bevy::{
    color::palettes::css::*,
    picking::backend::ray::RayMap,
    prelude::*,
    render::{camera::Viewport, view::RenderLayers},
    window::{PresentMode, WindowResized},
};

mod resources;
use resources::*;

mod components;
use components::*;

const PRIMARY_LAYER: usize = 0;
const SECONDARY_LAYER: usize = 1;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MeshPickingPlugin)
        .init_resource::<Materials>()
        .init_resource::<Square>()
        .add_systems(Startup, setup_cameras)
        .add_systems(
            Startup,
            ((setup_colors, setup_square), create_board_left).chain(),
        )
        .add_systems(Update, (sync_cameras_viewports, print_rays_constructor()))
        .run();
}

fn create_color(asset_server: &AssetServer, color: Srgba) -> Handle<ColorMaterial> {
    asset_server.add(ColorMaterial {
        color: color.into(),
        ..default()
    })
}

fn setup_colors(asset_server: Res<AssetServer>, mut color_resource: ResMut<Materials>) {
    color_resource.red = create_color(&asset_server, RED);
    color_resource.green = create_color(&asset_server, GREEN);
    color_resource.yellow = create_color(&asset_server, YELLOW);
    color_resource.blue = create_color(&asset_server, BLUE);
}

fn setup_square(asset_server: Res<AssetServer>, mut square_resource: ResMut<Square>) {
    square_resource.mesh = asset_server.add(Rectangle::from_length(32.0).into());
}

fn setup_cameras(mut commands: Commands, window_query: Query<&Window>) -> Result {
    let window = window_query.single()?;
    let window_size = window.resolution.physical_size();
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(DARK_GRAY.into()),
            viewport: Some(Viewport {
                physical_size: UVec2::new(window_size.x / 2, window_size.y),
                physical_position: UVec2::new(20, 0),
                ..default()
            }),
            order: 0,
            ..default()
        },
        RenderLayers::layer(PRIMARY_LAYER),
        MeshPickingCamera,
        PrimaryCamera,
        IsDefaultUiCamera,
    ));
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(DARK_GRAY.into()),
            viewport: Some(Viewport {
                physical_size: UVec2::new(window_size.x / 2, window_size.y),
                physical_position: UVec2::new(window_size.x / 2, 0),
                ..default()
            }),
            order: 1,
            ..default()
        },
        MeshPickingCamera,
        RenderLayers::layer(SECONDARY_LAYER),
        SecondaryCamera,
    ));
    Ok(())
}

fn sync_cameras_viewports(
    mut primary_camera_query: Query<&mut Camera, With<IsDefaultUiCamera>>,
    mut secondary_camera_query: Query<&mut Camera, Without<IsDefaultUiCamera>>,
    mut resize_reader: EventReader<WindowResized>,
    window_query: Query<&Window>,
) -> Result {
    for event in resize_reader.read() {
        let window = window_query.get(event.window)?;
        let window_size = window.physical_size();
        let mut primary_camera = primary_camera_query.single_mut()?;
        if let Some(ref mut viewport) = primary_camera.viewport {
            viewport.physical_size = UVec2::new(window_size.x / 2, window_size.y);
        }
        let mut secondary_camera = secondary_camera_query.single_mut()?;
        if let Some(ref mut viewport) = secondary_camera.viewport {
            viewport.physical_size = UVec2::new(window_size.x / 2, window_size.y);
            viewport.physical_position = UVec2::new(window_size.x / 2, 0);
        }
    }
    Ok(())
}

fn create_board_left(
    mut commands: Commands,
    square_resource: Res<Square>,
    color_resource: Res<Materials>,
) {
    commands
        .spawn((
            Mesh2d(square_resource.mesh.clone()),
            MeshMaterial2d(color_resource.blue.clone()),
            RenderLayers::from_layers(&[PRIMARY_LAYER, SECONDARY_LAYER]),
        ))
        .observe(on_over)
        .observe(on_leave);
}

fn on_over(
    over: Trigger<Pointer<Over>>,
    mut material: Query<&mut MeshMaterial2d<ColorMaterial>>,
    color_resource: Res<Materials>,
) {
    if let Ok(mut material) = material.get_mut(over.target) {
        material.0 = color_resource.red.clone();
    }
}

fn on_leave(
    leave: Trigger<Pointer<Out>>,
    mut material: Query<&mut MeshMaterial2d<ColorMaterial>>,
    color_resource: Res<Materials>,
) {
    if let Ok(mut material) = material.get_mut(leave.target) {
        material.0 = color_resource.blue.clone();
    }
}

fn print_rays_constructor() -> impl FnMut(Res<Time>, Res<RayMap>) {
    let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
    move |time, rays| {
        timer.tick(time.delta());
        if timer.just_finished() {
            println!("{:?}", rays);
        }
    }
}
