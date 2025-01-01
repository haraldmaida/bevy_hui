#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(rustdoc::redundant_explicit_links)]
#![doc = include_str!("../../../README.md")]

use bevy::{app::{App, Plugin, Update}, prelude::{ImageNode, Query, Res}, time::Time};
use data::{ActiveAnimation, AnimationDirection};
use styles::HtmlStyle;

mod auto;
mod bindings;
mod build;
mod compile;
mod data;
mod error;
mod load;
mod parse;
mod styles;
mod util;

pub mod prelude {
    pub use crate::auto::{AutoLoadState, HuiAutoLoadPlugin};
    pub use crate::bindings::{
        ComponentBindings, FunctionBindings, HtmlComponents, HtmlFunctions, UiChangedEvent,
    };
    pub use crate::build::{
        HtmlNode, OnUiChange, OnUiEnter, OnUiExit, OnUiPress, OnUiSpawn, Tags, TemplateProperties,
        TemplateScope, UiId, UiTarget, UiWatch,
    };
    pub use crate::compile::{CompileContextEvent, CompileNodeEvent};
    pub use crate::data::{Action, Attribute, HtmlTemplate, NodeType, StyleAttr};
    pub use crate::error::ParseError;
    pub use crate::error::VerboseHtmlError;
    pub use crate::parse::parse_template;
    pub use crate::styles::{HoverTimer, HtmlStyle, InteractionTimer, PressedTimer, UiActive};
    pub use crate::HuiPlugin;
}

fn run_animations(
    time: Res<Time>,
    mut query: Query<(&mut ActiveAnimation, &mut ImageNode, &HtmlStyle)>,
) {
    for (mut active_animation, mut node, style) in query.iter_mut() {
        if active_animation.iterations == 0 {
            println!("Out of iterations");
            continue;
        }

        if style.computed.duration > 0.0 {
            active_animation.duration = active_animation.duration - time.delta_secs();

            if active_animation.duration <= 0.0 {
                println!("Duration up");
                continue;
            }
        }

        active_animation.timer.tick(time.delta());

        if active_animation.timer.finished() {
            println!("Frame timer up");

            let atlas = node.texture_atlas.as_mut().unwrap();
            let atlas_details = style.computed.atlas.as_ref().unwrap();

            if style.computed.frames.len() == 0 {
                let frame_count = (atlas_details.columns * atlas_details.rows) as usize;

                match active_animation.direction {
                    AnimationDirection::Forward => {
                        if atlas.index == frame_count - 1 {
                            if style.computed.direction == AnimationDirection::AlternateForward || style.computed.direction == AnimationDirection::AlternateReverse{
                                active_animation.direction = AnimationDirection::Reverse;
                                active_animation.frame = frame_count - 2;
                            } else {
                                active_animation.frame = 0;
                            }
                            active_animation.iterations = active_animation.iterations - 1;
                        } else {
                            active_animation.frame = active_animation.frame + 1;
                        }
                    }
                    AnimationDirection::Reverse => {
                        if atlas.index == 0 {
                            if style.computed.direction == AnimationDirection::AlternateForward || style.computed.direction == AnimationDirection::AlternateReverse{
                                active_animation.direction = AnimationDirection::Forward;
                                active_animation.frame = 1;
                            } else {
                                active_animation.frame = frame_count - 1;
                            }
                            active_animation.iterations = active_animation.iterations - 1;
                        } else {
                            active_animation.frame = active_animation.frame - 1;
                        }
                    }
                    _ => (),
                }

                node.texture_atlas.as_mut().unwrap().index = active_animation.frame;
            } else {
                let frame_count = style.computed.frames.len();

                match active_animation.direction {
                    AnimationDirection::Forward => {
                        if active_animation.frame == frame_count - 1 {
                            if style.computed.direction == AnimationDirection::AlternateForward || style.computed.direction == AnimationDirection::AlternateReverse{
                                active_animation.direction = AnimationDirection::Reverse;
                                active_animation.frame = frame_count - 2;
                            } else {
                                active_animation.frame = 0;
                            }
                            active_animation.iterations = active_animation.iterations - 1;
                        } else {
                            active_animation.frame = active_animation.frame + 1;
                        }
                    },
                    AnimationDirection::Reverse => {
                        if active_animation.frame == 0 {
                            if style.computed.direction == AnimationDirection::AlternateForward || style.computed.direction == AnimationDirection::AlternateReverse{
                                active_animation.direction = AnimationDirection::Forward;
                                active_animation.frame = 1;
                            } else {
                                active_animation.frame = frame_count - 1;
                            }
                            active_animation.iterations = active_animation.iterations - 1;
                        } else {
                            active_animation.frame = active_animation.frame - 1;
                        }
                    }
                    _ => (),
                }

                node.texture_atlas.as_mut().unwrap().index = style.computed.frames[active_animation.frame] as usize;
            }
        }
    }
}

pub struct HuiPlugin;
impl Plugin for HuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            load::LoaderPlugin,
            build::BuildPlugin,
            bindings::BindingPlugin,
            styles::TransitionPlugin,
            compile::CompilePlugin,
        )).add_systems(Update, run_animations);
    }
}
