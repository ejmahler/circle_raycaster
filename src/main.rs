#![feature(rust_2018_preview)]
#![feature(generators, generator_trait)]

use image::{ImageBuffer, Rgb};

use palette::{Hsv, Srgb};

mod pixel_math;
mod cast;
mod config;

use crate::config::RaycastConfig;
use crate::cast::{CastGrid, CastResults};



fn main() {
    let config_data = RaycastConfig::new(20000, false);

    let mut img = ImageBuffer::from_pixel(config_data.output_dim as u32, config_data.output_dim as u32, Rgb([0,0,0]));

    let mut cast_results = CastResults::new();
    let mut raycaster = CastGrid::new(&config_data);
    
    for (ray_index, ray_origin) in config_data.origin_pixels.iter().enumerate() {
        raycaster.cast(ray_origin, &config_data.target_pixel, &mut cast_results);
        for (result_index, result_pixel) in cast_results.out_pixels.iter().enumerate() {
            let color = color_for_ray_distance(&config_data, &cast_results, ray_index, result_index);

            img.put_pixel(result_pixel.x as u32, result_pixel.y as u32, color);
        }
    }
    img.save("test.png").unwrap();
}

// fn color_for_iteration(config_data: &RaycastConfig, results: &CastResults, ray_index: usize, cast_result_index: usize) -> Rgb<u8> {
//     let color = Hsv::new((ray_index as f32) / total_rays_float * 360.0, 1.0, 1.0);
//     let color_rgb: Srgb = color.into();

//     return Rgb([
//         percent_to_u8(color_rgb.red),
//         percent_to_u8(color_rgb.green),
//         percent_to_u8(color_rgb.blue),
//     ])
// }

#[allow(unused)]
fn ease_out_quad(val: f32) -> f32 {
    val * (2.0 - val)
}
#[allow(unused)]
fn ease_in_quad(val: f32) -> f32 {
    val * val
}

fn color_for_ray_distance(_config_data: &RaycastConfig, results: &CastResults, _ray_index: usize, _cast_result_index: usize) -> Rgb<u8> {

    let ray_percent = _ray_index as f32 / _config_data.total_rays_float;
    let distance_percent = ease_out_quad(1.0 - results.cast_distance_percent) * 0.97;
    let pixel_percent = ease_in_quad((_cast_result_index + 1) as f32 / results.pixel_size_float);

    let color = Hsv::new(ray_percent * 360.0, 1.0, pixel_percent);
    let color_rgb: Srgb = color.into();

    return Rgb([
        percent_to_u8(color_rgb.red),
        percent_to_u8(color_rgb.green),
        percent_to_u8(color_rgb.blue),
    ])
}


fn percent_to_u8(percent: f32) -> u8 {
    let val = percent * 255.0;
    if val < 0.0 {
        0
    } else if val > 255.0 {
        255
    } else {
        val as u8
    }
}