use image::{GrayImage, Luma};
use tracing::{event, Level};
use opencv::{core::{Mat, KeyPoint, Vector}};

pub fn get_fast_keypoints(img: &GrayImage) -> Result<Vector<KeyPoint>, opencv::Error> {
    let mut keypoints : Vector<KeyPoint>= Vec::new().into();
    let img_rows: Vec<Vec<u8>> = img.rows().map(|row| row.map(|&Luma(pix)| pix[0]).collect::<Vec<u8>>()).collect();
    let img_rows_slices: Vec<&[u8]> = img_rows.iter().map(AsRef::as_ref).collect();
    let threshold = 20;
    let nonmax_suppression = true;
    let img_mat: Mat = Mat::from_slice_2d::<u8>(&img_rows_slices)?;
    opencv::features2d::fast(&img_mat, &mut keypoints, threshold, nonmax_suppression).expect("FAST failed");

    Ok(keypoints)
}