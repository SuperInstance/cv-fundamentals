//! # cv-fundamentals
//!
//! Computer vision fundamentals — image processing, feature detection, and geometric vision.

pub mod image;
pub mod filter;
pub mod morphology;
pub mod feature;
pub mod transform;
pub mod segmentation;
pub mod stereo;
pub mod optical_flow;

pub use image::{GrayImage, ImageError};
