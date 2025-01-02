use bevy::prelude::*;
use bevy_hui::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin {
                default_sampler: bevy::image::ImageSamplerDescriptor::nearest(),
            }),
            HuiPlugin,
        ))
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(mut cmd: Commands, server: Res<AssetServer>, mut html_comps: HtmlComponents) {
    cmd.spawn(Camera2d);
    
    html_comps.register("animation", server.load("demo/animation.html"));

    cmd.spawn(HtmlNode(server.load("demo/animation.html")));
}