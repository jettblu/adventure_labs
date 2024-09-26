use drm_fourcc::DrmFourcc;

use image::RgbImage;
use image::ColorType;
use image::ImageBuffer;

use std::time::Duration;

use libcamera::{
    camera::CameraConfigurationStatus,
    camera_manager::CameraManager,
    framebuffer::AsFrameBuffer,
    framebuffer_allocator::{ FrameBuffer, FrameBufferAllocator },
    framebuffer_map::MemoryMappedFrameBuffer,
    pixel_format::PixelFormat,
    properties,
    stream::StreamRole,
    geometry::Size,
};

const PIXEL_FORMAT_RGB888: PixelFormat = PixelFormat::new(DrmFourcc::Rgb888 as u32, 0);

pub fn save_photo_to_file(filename: &str) {
    // Set size.
    let size = Size { width: 800, height: 600 };

    // Initialize the camera manager and select the first camera.
    let mgr = CameraManager::new().unwrap();
    let cameras = mgr.cameras();
    let cam = cameras.get(0).expect("No cameras found");
    println!("Using camera: {}", *cam.properties().get::<properties::Model>().unwrap());

    // Acquire the camera.
    let mut cam = cam.acquire().expect("Unable to acquire camera");

    // This will generate default configuration for each specified role.
    let mut cfgs = cam.generate_configuration(&[StreamRole::ViewFinder]).unwrap();

    // Set the desired pixel format and size for the configuration.
    cfgs.get_mut(0).unwrap().set_pixel_format(PIXEL_FORMAT_RGB888);
    cfgs.get_mut(0).unwrap().set_size(size);

    // Print the generated configuration.
    println!("Generated config: {:#?}", cfgs);

    // Validate the generated configuration.
    match cfgs.validate() {
        CameraConfigurationStatus::Valid => println!("Camera configuration valid!"),
        CameraConfigurationStatus::Adjusted =>
            println!("Camera configuration was adjusted: {:#?}", cfgs),
        CameraConfigurationStatus::Invalid => panic!("Error validating camera configuration"),
    }
    cam.configure(&mut cfgs).expect("Unable to configure camera");

    // Allocate frame buffers for the stream.
    let mut alloc = FrameBufferAllocator::new(&cam);
    let stream = cfgs
        .get(0)
        .expect("Failed to get stream configuration")
        .stream()
        .expect("Failed to get stream");
    let buffers = alloc.alloc(&stream).expect("Failed to allocate buffers");
    println!("Allocated {} buffers", buffers.len());

    // Convert FrameBuffer to MemoryMappedFrameBuffer, which allows reading &[u8]
    let buffers = buffers
        .into_iter()
        .map(|buf| MemoryMappedFrameBuffer::new(buf).expect("Failed to map framebuffer"))
        .collect::<Vec<_>>();

    // Create capture requests and attach buffers.
    let mut reqs = buffers
        .into_iter()
        .map(|buf| {
            let mut req = cam.create_request(None).expect("Failed to create request");
            req.add_buffer(&stream, buf).expect("Failed to add buffer to request");
            req
        })
        .collect::<Vec<_>>();

    // Completed capture requests are returned as a callback.
    let (tx, rx) = std::sync::mpsc::channel();
    cam.on_request_completed(move |req| {
        tx.send(req).expect("Failed to send completed request");
    });

    // Start the camera and queue a single capture request.
    cam.start(None).expect("Failed to start camera");
    cam.queue_request(reqs.pop().expect("Failed to pop request")).expect("Failed to queue request");

    println!("Waiting for camera request execution");
    let req = rx.recv_timeout(Duration::from_secs(2)).expect("Camera request failed");

    println!("Camera request {:?} completed!", req);
    println!("Metadata: {:#?}", req.metadata());

    // Retrieve and process the framebuffer data from the completed request.
    let framebuffer: &MemoryMappedFrameBuffer<FrameBuffer> = req
        .buffer(&stream)
        .expect("Failed to get framebuffer");
    println!("FrameBuffer metadata: {:#?}", framebuffer.metadata());

    // RGB data is interleaved as a single plane.
    let planes = framebuffer.data();
    let image_plane = match planes.get(0) {
        Some(plane) => plane,
        None => {
            eprintln!("RGB data is not available");
            return;
        }
    };
    let mut rgb_vec = image_plane.to_vec();
    swap_channels(&mut rgb_vec);
    // Create an ImageBuffer from the raw RGB data
    let img: RgbImage = ImageBuffer::from_raw(size.width, size.height, rgb_vec).expect(
        "Unable to form the image buffer"
    );

    // Save the buffer to a PNG file.
    image::save_buffer(&filename, &img, size.width, size.height, ColorType::Rgb8).unwrap();

    println!("PNG file saved to {}", &filename);

    // Everything is cleaned up automatically by Drop implementations.
}

// function that swaps channels of rgb image
fn swap_channels(img: &mut Vec<u8>) {
    for i in 0..img.len() / 3 {
        let r = img[i * 3];
        let b = img[i * 3 + 2];
        img[i * 3] = b;
        img[i * 3 + 2] = r;
    }
}
