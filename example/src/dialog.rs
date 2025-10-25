use bevy::{image::ImageSamplerDescriptor, prelude::*};
use bevy_hui::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor::nearest(),
            }),
            HuiPlugin,
        ))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands, server: Res<AssetServer>, mut html_funcs: HtmlFunctions) {
    cmd.spawn(Camera2d);
    cmd.spawn(HtmlNode(server.load("demo/dialog.html")));
    html_funcs.register("press", press);
}
fn press(In(entity): In<Entity>) {
    info!("{:?}:press", entity)
}
