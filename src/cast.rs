
use cgmath::Point2;
use crate::pixel_math;
use crate::config::RaycastConfig;

pub struct CastResults {
    pub out_pixels: Vec<Point2<i32>>,
    pub pixel_size_float: f32,
    pub cast_distance: f32,
    pub cast_distance_percent: f32,
}
impl CastResults {
    pub fn new() -> Self {
        Self {
            out_pixels: Vec::new(),
            pixel_size_float: 0.0,
            cast_distance: 0.0,
            cast_distance_percent: 0.0,
        }
    }
}

pub struct CastGrid<'a> {
    config: &'a RaycastConfig,
    grid: Vec<bool>,
}

impl<'a> CastGrid<'a> {
    pub fn new(config: &'a RaycastConfig) -> Self {
        Self {
            config,
            grid: vec![false; config.output_dim * config.output_dim],
        }
    }

    pub fn cast(&mut self, from: &Point2<i32>, to: &Point2<i32>, result: &mut CastResults) {
        result.out_pixels.clear();
        result.out_pixels.reserve(self.config.radius);
        for pixel in pixel_math::iter_pixels_in_line(*from, *to).take_while(|pixel| self.try_mark(pixel)) {
            result.out_pixels.push(pixel);
        }
        result.pixel_size_float = result.out_pixels.len() as f32;

        let distance_sq = if result.out_pixels.len() == 0 {
            0
        } else {
            let offset = result.out_pixels.first().unwrap() - result.out_pixels.last().unwrap();
            offset.x * offset.x + offset.y * offset.y
        };

        result.cast_distance = (distance_sq as f32).sqrt();
        result.cast_distance_percent = result.cast_distance / self.config.radius_float;
    }

    fn try_mark(&mut self, p: &Point2<i32>) -> bool {
        let index = p.y as usize * self.config.output_dim + (p.x as usize);
        let old = self.grid[index];
        self.grid[index] = true;
        !old
    }
}