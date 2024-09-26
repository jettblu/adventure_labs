use camera_capture::{ save_photo_to_file };

fn main() {
    let filename = "test_rgb888_capture.png";
    save_photo_to_file(filename);
}
