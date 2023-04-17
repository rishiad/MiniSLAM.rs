// 3x3 patch for a keypoint o
// x x x
// x o x
// x x x

use opencv::{core::{KeyPoint, Vector}, prelude::{KeyPointTraitConst}, Error};
use image::{GrayImage};
use tracing::{event, Level};

pub fn harris_corner_response(fast_keypoints: &Vector<KeyPoint>, block_size: i16, img: &GrayImage) -> Result<Vector<KeyPoint>, Error> {

    let mut ranked_keypoints : Vector<KeyPoint> = Default::default();
    let mut keypoints_response : Vector<KeyPoint> = Default::default();
    let half_block = block_size / 2;
    let mut surrounding_pixels : Vec<i32> = Vec::new();

    let harris_constant = 0.04;

    // Create Sobel kernels
    let soble_x_kernal: Vec<i32> = vec![1, 0, -1, 2, 0, -2, 1, 0, -1];
    let soble_y_kernal: Vec<i32> = vec![-1, -2, -1, 0, 0, 0, 1, 2, 1];

    let mut soble_product_sum: i32 = 0;
    // Iterate over all keypoints and compute sobel values
    for kp in fast_keypoints.iter() {
        let mut soble_x_sum: i32 = 0;
        let mut soble_y_sum: i32 = 0;
        let x = kp.pt().x as i32;
        let y = kp.pt().y as i32;
    
        for j in (y as i32 - half_block as i32).max(0)..=(y as i32 + half_block as i32).min(img.height() as i32 - 1) {
            for i in (x as i32 - half_block as i32).max(0)..=(x as i32 + half_block as i32).min(img.width() as i32 - 1) {
                surrounding_pixels.push(img.get_pixel(i as u32, j as u32)[0].try_into().unwrap());
            }
        }
    
        for (i, pixel) in surrounding_pixels.iter().enumerate() {
            soble_x_sum += pixel * soble_x_kernal[i] as i32;
            soble_y_sum += pixel * soble_y_kernal[i] as i32;
        }

        // Calculate M matrix elements
        let m11: i64 = (soble_x_sum * soble_x_sum).into();
        let m12:i64 = (soble_x_sum * soble_y_sum).into();
        let m22: i64 = (soble_y_sum * soble_y_sum).into();
        event!(Level::INFO, "m11: {}", m11);
        event!(Level::INFO, "m12: {}", m12);
        // Calculate Harris response
        let det_m: i64 = m11 * m12 * m22;
        event!(Level::INFO, "det_m: {}", det_m); 
        let trace_m = m11 + m22;
        let harris_response = (det_m as f64) - harris_constant * ((trace_m as f64) * (trace_m as f64));
        
        let new_kp = KeyPoint::new_point(kp.pt(), kp.size(), kp.angle(), harris_response as f32, kp.octave(), kp.class_id());
        
        keypoints_response.push(new_kp?);
        surrounding_pixels.clear()
    }
    
    // ranked keypoints using the soble score
    let mut sort_keypoint_vector = keypoints_response.to_vec();
    sort_keypoint_vector.sort_by(|a, b| b.response().partial_cmp(&a.response()).unwrap_or(std::cmp::Ordering::Equal));
    for kp in sort_keypoint_vector.iter() {
        ranked_keypoints.push(kp.clone());
    }

    Ok(ranked_keypoints)
}

