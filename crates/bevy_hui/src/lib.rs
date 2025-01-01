#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(rustdoc::redundant_explicit_links)]
#![doc = include_str!("../../../README.md")]

use bevy::{app::{App, Plugin, Update}, prelude::{ImageNode, Query, Res}, time::Time};
use data::{AnimationDirection, AnimationDuration, AnimationFrame, AnimationIterations, AnimationTimer};
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
    mut query: Query<(&mut AnimationTimer, &mut AnimationDirection, &mut AnimationDuration, &mut AnimationIterations, &mut AnimationFrame, &mut ImageNode, &HtmlStyle)>,
) {
    for (mut timer, mut direction, mut duration, mut iterations, mut frame, mut node, style) in query.iter_mut() {
        if iterations.0 == 0 {
            continue;
        }

        duration.0 = duration.0 - (time.delta_secs() * 1000.0) as i64;

        if duration.0 > style.computed.duration {
            continue;
        }

        timer.0.tick(time.delta());

        if timer.0.finished() {
            iterations.0 = iterations.0 - 1;

            let atlas = node.texture_atlas.as_mut().unwrap();
            let atlas_details = style.computed.atlas.as_ref().unwrap();
            
            if style.computed.frames.len() == 0 {
                let frame_count = (atlas_details.columns * atlas_details.rows) as usize;

                match *direction {
                    AnimationDirection::Forward => {
                        if atlas.index == frame_count - 1 {
                            if style.computed.direction == AnimationDirection::AlternateForward || style.computed.direction == AnimationDirection::AlternateReverse{
                                *direction = AnimationDirection::Reverse;
                                frame.0 = frame_count - 2;
                            } else {
                                frame.0 = 0;
                            }
                        } else {
                            frame.0 = frame.0 + 1;
                        }
                    }
                    AnimationDirection::Reverse => {
                        if atlas.index == 0 {
                            if style.computed.direction == AnimationDirection::AlternateForward || style.computed.direction == AnimationDirection::AlternateReverse{
                                *direction = AnimationDirection::Forward;
                                frame.0 = 1;
                            } else {
                                frame.0 = frame_count - 1;
                            }
                        } else {
                            frame.0 = frame.0 - 1;
                        }
                    }
                    _ => (),
                }
            } else {
                let frame_count = style.computed.frames.len();

                match *direction {
                    AnimationDirection::Forward => {
                        if frame.0 == frame_count - 1 {
                            if style.computed.direction == AnimationDirection::AlternateForward || style.computed.direction == AnimationDirection::AlternateReverse{
                                *direction = AnimationDirection::Reverse;
                                frame.0 = style.computed.frames[frame_count - 2] as usize;
                            } else {
                                frame.0 = style.computed.frames[0] as usize;
                            }
                        } else {
                            frame.0 = style.computed.frames[frame_count + 1] as usize;
                        }
                    },
                    AnimationDirection::Reverse => {
                        if frame.0 == 0 {
                            if style.computed.direction == AnimationDirection::AlternateForward || style.computed.direction == AnimationDirection::AlternateReverse{
                                *direction = AnimationDirection::Forward;
                                frame.0 = style.computed.frames[1] as usize;
                            } else {
                                frame.0 = style.computed.frames[frame_count - 1] as usize;
                            }
                        } else {
                            frame.0 = style.computed.frames[frame_count - 1] as usize;
                        }
                    }
                    _ => (),
                }
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
