use image::{RgbImage, Luma, GrayImage, Rgb};
use opencv::{core::{Vector, KeyPoint}, prelude::KeyPointTraitConst};

pub fn image_overlay( mut img: GrayImage, frame_count: i64, fskeypoints: Vector<KeyPoint>, harris_keypoints: Vector<KeyPoint> ) -> Result<image::RgbImage, Box<dyn std::error::Error>> {
    let frame_count_str = frame_count.to_string();

    let (width, height) = img.dimensions();
    let mut rgb_image = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let gray_pixel = img.get_pixel(x, y);
            let rgb_pixel = Rgb([gray_pixel[0], gray_pixel[0], gray_pixel[0]]);
            rgb_image.put_pixel(x, y, rgb_pixel);
        }
    };

    let offset_x = 5; // Adjust this value to change the horizontal position of the text
    let offset_y = 5; // Adjust this value to change the vertical position of the text
    let font_data: &[u8] = include_bytes!("../../font/Roboto/Roboto-Regular.ttf");
    let font: rusttype::Font<'static> = rusttype::Font::try_from_bytes(font_data).unwrap();

    imageproc::drawing::draw_text_mut(
        &mut rgb_image,
        image::Rgb([255u8, 255u8, 255u8]),
        offset_x,
        offset_y,
        rusttype::Scale::uniform(20.0),
        &font,
        &frame_count_str

    );

    for kp in fskeypoints.iter() {
        let x = kp.pt().x as i32;
        let y = kp.pt().y as i32;
        let rect = imageproc::rect::Rect::at(x, y).of_size(5, 5);
        let color = image::Rgb([255u8, 255u8, 255u8]);
        imageproc::drawing::draw_hollow_rect_mut(&mut rgb_image, rect, color);
    };

    for kp in harris_keypoints.iter() {
        let x = kp.pt().x as i32;
        let y = kp.pt().y as i32;
        let rect = imageproc::rect::Rect::at(x, y).of_size(5, 5);
        let color = image::Rgb([54u8, 239u8, 17u8]); 
        imageproc::drawing::draw_hollow_rect_mut(&mut rgb_image, rect, color);
    }

    Ok(rgb_image)
}