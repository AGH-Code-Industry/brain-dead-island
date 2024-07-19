//! Rendering pipeline for bdi.

pub mod game_display;

pub mod camera;
pub mod renderable_objects;
/// SDL2 implentation of the rendering pipeline.
pub mod sdl;
/// Rendering pipeline structures.
/// Implementation agnostic.
pub mod structures;
/// Rendering pipeline traits.
pub mod traits;
