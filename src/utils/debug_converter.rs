use opencv::{core::KeyPoint, prelude::KeyPointTraitConst, core::Vector};

pub fn convert_keypoints_to_tuples(keypoints: &Vector<KeyPoint>) -> Vec<(f32, f32, f32)> {
    keypoints.iter().map(|kp| (kp.pt().x, kp.pt().y, kp.response())).collect()
}