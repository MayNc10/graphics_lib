//! The three_d module provides support for loading and rendering 3D models.
//!
//! Currently, the only supported file format for models is OBJ/MTL.
//!
//! The engine can render objects using a variety of shaders, such as Blinn-Phong or Gouraud.
//! The engine can also render objects using deferred Blinn-Phong rendering, allowing for a arbitrary number of lights.
//! This module also provides a raytracing engine for rendering scenes
//!
pub mod animation;
pub mod buffer;
pub mod scene;
pub mod shaders;
pub mod shape;
pub mod vao;
pub mod lights;
pub mod raytracing;
//pub mod teapot;