use camera_capture::{ save_photo_to_file };

fn main() {
    let filename = "test_rgb888_capture.png";
    let filename_compressed = "test_rgb888_capture.webp";
    save_photo_to_file(filename, Some(filename_compressed));
}
