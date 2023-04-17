mod data_loaders;
mod feature_extraction;
mod mapping;

mod utils;
mod overlay;

use std::{env}; 
use minifb::{Window, WindowOptions, Key};
use opencv::core::{Vector, KeyPoint};
use tracing::{event, Level};
use tracing_subscriber;

fn main() {

    // Set up logging
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish(),
    )
    .expect("Failed to set tracing subscriber");

    // Get path to video 
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    event!(Level::INFO, "Loading video from {}", path);

    event!(Level::INFO, "Loading video frames and timestamps...");
    let cam_video = data_loaders::image_loader::stero_img_loader(&path).expect("Failed to load video frames and timestamps from given path");
    
    // Display images and timestamps in a minifb window 
    event!(Level::INFO, "Setting up cam0 window");
    let mut cam0_window = Window::new("cam0", cam_video[0].1.width() as usize, cam_video[0].1.height() as usize, WindowOptions::default()).unwrap();
    
    for (t0, cam0_img) in cam_video.iter() {
        let fast_keypoints = feature_extraction::fast_detector::get_fast_keypoints(&cam0_img).unwrap();
        let debug_keypoints = utils::debug_converter::convert_keypoints_to_tuples(&fast_keypoints);
        let harris_corner_response = feature_extraction::harris_corner::harris_corner_response(&fast_keypoints, 3, &cam0_img).unwrap();
        let filtered_keypoints = harris_corner_response.iter().take(200).clone().collect::<Vector<KeyPoint>>();
        // event!(Level::INFO, "FAST keypoints: {:?}", debug_keypoints);
        let overlayed_img = overlay::overlay::image_overlay(cam0_img.clone(), t0.clone().unwrap(), fast_keypoints,filtered_keypoints).unwrap();
        let cam0_buffer = utils::buffer_converter::gray_imagto_minifb_buffer(&overlayed_img);
        event!(Level::INFO, "cam0 timestamp: {:?}", t0.unwrap());
        cam0_window.update_with_buffer(&cam0_buffer, cam0_img.width() as usize, cam0_img.height() as usize).unwrap();
    
        if cam0_window.is_key_down(Key::Escape) {
            break;
        };
    
        std::thread::sleep(std::time::Duration::from_millis(30));
        
    }
}
