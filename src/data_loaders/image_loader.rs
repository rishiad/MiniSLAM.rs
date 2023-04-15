use tracing::{event, Level};
use image::{self};
use ffmpeg::{self};

pub fn stero_img_loader(video_file: &str) -> Result<Vec<(Option<i64>, image::GrayImage)>, Box<dyn std::error::Error>> {

    ffmpeg::init().unwrap();

    let mut format_input = ffmpeg::format::input(&video_file)?;
    let input_stream = format_input.streams().best(ffmpeg::media::Type::Video).unwrap();
    let mut decoder = input_stream.codec().decoder().video()?;
    let mut img_timestamps = Vec::new();
    let input_stream_index = input_stream.index();
    let input_timebase = input_stream.time_base();
    drop(input_stream); 

    for (stream, packet) in format_input.packets() {
        if stream.index() == input_stream_index {
            let mut decoded = ffmpeg::util::frame::video::Video::empty();
            let timestamp = packet.pts().map(|pts| (pts as f64 * input_timebase.0 as f64 / input_timebase.1 as f64 * 1_000_000.0) as i64);
            event!(Level::INFO, "Decoding video frame with timestamp: {:?}", timestamp.unwrap());
            decoder.decode(&packet, &mut decoded)?;

            let frame_width = decoded.width() as u32;
            let frame_height = decoded.height() as u32;

            if frame_width == 0 || frame_height == 0 {
                event!(Level::INFO, "Skipping frame with timestamp: {:?}", timestamp.unwrap());
                continue;
            }

            let frame_data = decoded.data(0);
            let frame_width = decoded.width() as u32;
            let frame_height = decoded.height() as u32;

            let gray_img_data: Vec<u8> = frame_data.iter().cloned().collect();
            let gray_img_buffer = image::GrayImage::from_raw(frame_width, frame_height, gray_img_data).unwrap();

            img_timestamps.push((timestamp, gray_img_buffer));
            event!(Level::INFO, "Decoded video frame with timestamp: {:?}", timestamp.unwrap() );
        }
    }

    event!(Level::INFO, "Loaded {} video frames and timestamps successfully", img_timestamps.len());
    Ok(img_timestamps)
}