//! Rendering pipeline for bdi.

pub mod game_display;

pub mod camera;
/// SDL2 implentation of the rendering pipeline.
pub mod sdl;
/// Rendering pipeline structures.
/// Implementation agnostic.
pub mod structures;
/// Rendering pipeline traits.
pub mod traits;
mod renderers;
mod displays;
pub mod renderable_objects;

