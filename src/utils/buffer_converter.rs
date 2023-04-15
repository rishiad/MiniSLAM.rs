pub fn gray_imagto_minifb_buffer(img: &image::GrayImage) -> Vec<u32> {
    let buffer = img.enumerate_pixels()
        .map(|(_, _, pixel)| {
            let pixel = pixel[0];
            let pixel = pixel as u32;
            let pixel = pixel << 16 | pixel << 8 | pixel;
            pixel
        })
        .collect();
    
    return buffer;
}