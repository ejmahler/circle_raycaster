use cgmath::Point2;
use rand::{thread_rng, Rng};

use crate::pixel_math;

pub struct RaycastConfig {
    pub radius: usize,
    pub radius_float: f32,
    pub target_pixel: Point2<i32>,
    pub origin_pixels: Vec<Point2<i32>>,
    pub total_rays_float: f32,
    pub output_dim: usize,
}

impl RaycastConfig {
    pub fn new(radius: usize, randomize_origins: bool) -> Self {
        let center = Point2::new(radius as i32, radius as i32);
        let mut origin_pixels: Vec<Point2<i32>> = pixel_math::iter_pixels_in_circle(radius as i32, center).collect();
        if randomize_origins {
            let mut rng = thread_rng();
            rng.shuffle(&mut origin_pixels);
        }

        Self {
            radius,
            radius_float: radius as f32,
            target_pixel: center,
            total_rays_float: origin_pixels.len() as f32,
            origin_pixels,
            output_dim: (radius * 2 + 1) as usize,
        }
    }
}