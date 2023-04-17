pub fn gray_imagto_minifb_buffer(img: &image::RgbImage) -> Vec<u32> {
    let buffer = img.enumerate_pixels()
        .map(|(_, _, pixel)| {
            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;
            r << 16 | g << 8 | b
        })
        .collect();
    
    return buffer;
}