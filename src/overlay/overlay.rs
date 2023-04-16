use image::{GrayImage, Luma};
use opencv::{core::{Vector, KeyPoint}, prelude::KeyPointTraitConst};

pub fn image_overlay( mut img: GrayImage, frame_count: i64, keypoints: Vector<KeyPoint> ) -> Result<image::GrayImage, Box<dyn std::error::Error>> {
    let frame_count_str = frame_count.to_string();

    let offset_x = 5; // Adjust this value to change the horizontal position of the text
    let offset_y = 5; // Adjust this value to change the vertical position of the text
    let font_data: &[u8] = include_bytes!("/Users/rishiadhikari/devs/RustMiniSLAM/font/Roboto/Roboto-Regular.ttf");
    let font: rusttype::Font<'static> = rusttype::Font::try_from_bytes(font_data).unwrap();

    imageproc::drawing::draw_text_mut(
        &mut img,
        Luma([255u8]),
        offset_x,
        offset_y,
        rusttype::Scale::uniform(20.0),
        &font,
        &frame_count_str

    );

    for kp in keypoints.iter() {
        let x = kp.pt().x as i32;
        let y = kp.pt().y as i32;
        let rect = imageproc::rect::Rect::at(x, y).of_size(5, 5);
        let color = image::Luma([255u8]);
        imageproc::drawing::draw_hollow_rect_mut(&mut img, rect, color);
    };

    Ok(img)
}