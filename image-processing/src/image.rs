use std::path::Path;
use image::io::Reader as ImageReader;
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb};

pub fn load_image_file(image_file_name: String) -> image::DynamicImage {
    println!("Loading image...");

    let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let assets_dir = project_dir.join("src/assets");
    let image_file_path = assets_dir.join(image_file_name);

    println!("Image file path: {:?}", image_file_path);

    let img = ImageReader::open(image_file_path)
        .unwrap()
        .decode()
        .unwrap();

    println!("Image loaded successfully!");

    img
}

pub fn save_image_file(image: &image::DynamicImage, image_file_name: String) {
    println!("Saving image...");

    let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let assets_dir = project_dir.join("src/assets");
    let image_file_path = assets_dir.join(image_file_name);

    println!("Image file path: {:?}", image_file_path);

    image.save(image_file_path).unwrap();

    println!("Image saved successfully!");
}

pub fn get_image_dimensions(image: &image::DynamicImage) -> (u32, u32) {
    let dimensions = image.dimensions();
    println!("Image dimensions: {:?}", dimensions);

    dimensions
}

pub fn get_image_pixels(image: &image::DynamicImage) -> Vec<u8> {
    let (width, height) = image.dimensions();
    let mut pixels = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            pixels.push(pixel[0]);
            pixels.push(pixel[1]);
            pixels.push(pixel[2]);
        }
    }

    pixels
}

pub fn apply_grayscale_filter(image: &mut image::DynamicImage) -> image::DynamicImage {
    let (width, height) = image.dimensions();

    for x in 0..width {
        for y in 0..height {
            let image::Rgb(data) = image.get_pixel(x, y).to_rgb();
            let gray = (data[0] as u32 + data[1] as u32 + data[2] as u32) / 3;
            let gray = gray as u8;
            image.put_pixel(x, y, image::Rgba([gray, gray, gray, 255]));
        }
    }

    image.clone()
}

pub fn apply_invert_filter(image: &mut image::DynamicImage) -> image::DynamicImage {
    let (width, height) = image.dimensions();

    for x in 0..width {
        for y in 0..height {
            let image::Rgb(data) = image.get_pixel(x, y).to_rgb();
            let inverted = image::Rgba([255 - data[0], 255 - data[1], 255 - data[2], 255]);
            image.put_pixel(x, y, inverted);
        }
    }

    image.clone()
}

pub fn apply_sepia_filter(image: &mut image::DynamicImage) -> image::DynamicImage {
    let (width, height) = image.dimensions();

    for x in 0..width {
        for y in 0..height {
            let image::Rgb(data) = image.get_pixel(x, y).to_rgb();
            let r = (data[0] as f32 * 0.393 + data[1] as f32 * 0.769 + data[2] as f32 * 0.189) as u8;
            let g = (data[0] as f32 * 0.349 + data[1] as f32 * 0.686 + data[2] as f32 * 0.168) as u8;
            let b = (data[0] as f32 * 0.272 + data[1] as f32 * 0.534 + data[2] as f32 * 0.131) as u8;
            image.put_pixel(x, y, image::Rgba([r, g, b, 255]));
        }
    }

    image.clone()
}

pub fn apply_threshold_filter(image: &mut image::DynamicImage, threshold: u8) -> image::DynamicImage {
    let (width, height) = image.dimensions();

    for x in 0..width {
        for y in 0..height {
            let image::Rgb(data) = image.get_pixel(x, y).to_rgb();
            let r = if data[0] > threshold { 255 } else { 0 };
            let g = if data[1] > threshold { 255 } else { 0 };
            let b = if data[2] > threshold { 255 } else { 0 };
            image.put_pixel(x, y, image::Rgba([r, g, b, 255]));
        }
    }

    image.clone()
}

pub fn apply_blur_filter(image: &mut image::DynamicImage, radius: u32) -> image::DynamicImage {
    let (width, height) = image.dimensions();
    let mut new_image = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            let mut count = 0;

            for dx in 0..radius {
                for dy in 0..radius {
                    let nx = x + dx;
                    let ny = y + dy;

                    if nx < width && ny < height {
                        let pixel = image.get_pixel(nx, ny);
                        r += pixel[0] as u32;
                        g += pixel[1] as u32;
                        b += pixel[2] as u32;
                        count += 1;
                    }
                }
            }

            r /= count;
            g /= count;
            b /= count;

            new_image.put_pixel(x, y, Rgb([r as u8, g as u8, b as u8]));
        }
    }

    new_image.into()
}

pub fn detect_edges(image: &mut image::DynamicImage) -> image::DynamicImage {
    let (width, height) = image.dimensions();
    let mut new_image = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;

            if x > 0 && y > 0 && x < width - 1 && y < height - 1 {
                let pixel = image.get_pixel(x, y);
                let left = image.get_pixel(x - 1, y);
                let right = image.get_pixel(x + 1, y);
                let top = image.get_pixel(x, y - 1);
                let bottom = image.get_pixel(x, y + 1);

                r = (left[0] as i32 - right[0] as i32).abs() + (top[0] as i32 - bottom[0] as i32).abs();
                g = (left[1] as i32 - right[1] as i32).abs() + (top[1] as i32 - bottom[1] as i32).abs();
                b = (left[2] as i32 - right[2] as i32).abs() + (top[2] as i32 - bottom[2] as i32).abs();
            }

            new_image.put_pixel(x, y, Rgb([r as u8, g as u8, b as u8]));
        }
    }

    new_image.into()
}

fn compare_neighbors(neighbors_level: u32, image: &image::DynamicImage, x: u32, y: u32) -> bool {
    let (width, height) = image.dimensions();

    let pixel = image.get_pixel(x, y);
    let left = image.get_pixel(x.saturating_sub(neighbors_level), y);
    let right = image.get_pixel((x + neighbors_level).min(width - 1), y);
    let top = image.get_pixel(x, y.saturating_sub(neighbors_level));
    let bottom = image.get_pixel(x, (y + neighbors_level).min(height - 1));

    let r = (left[0] as i32 - right[0] as i32).abs() + (top[0] as i32 - bottom[0] as i32).abs();
    let g = (left[1] as i32 - right[1] as i32).abs() + (top[1] as i32 - bottom[1] as i32).abs();
    let b = (left[2] as i32 - right[2] as i32).abs() + (top[2] as i32 - bottom[2] as i32).abs();

    r > 100 || g > 100 || b > 100
}

pub fn detect_footballers(image: &mut image::DynamicImage, level: u32) -> image::DynamicImage {
    let (width, height) = image.dimensions();
    let mut new_image = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let pixel = image.get_pixel(x, y).to_rgb();
            let [r, g, b] = pixel.0;

            let is_footballer = compare_neighbors(level, image, x, y);

            if is_footballer {
                new_image.put_pixel(x, y, Rgb([r, g, b]));
            } else {
                new_image.put_pixel(x, y, Rgb([0, 0, 0]));
            }
        }
    }

    new_image.into()
}

pub fn diff_images(image1: &image::DynamicImage, image2: &image::DynamicImage) -> image::DynamicImage {
    let (width1, height1) = image1.dimensions();
    let (width2, height2) = image2.dimensions();

    let width = width1.min(width2);
    let height = height1.min(height2);

    let mut new_image = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let pixel1 = image1.get_pixel(x, y).to_rgb();
            let pixel2 = image2.get_pixel(x, y).to_rgb();

            let [r1, g1, b1] = pixel1.0;
            let [r2, g2, b2] = pixel2.0;

            let r = (r1 as i32 - r2 as i32).abs();
            let g = (g1 as i32 - g2 as i32).abs();
            let b = (b1 as i32 - b2 as i32).abs();

            new_image.put_pixel(x, y, Rgb([r as u8, g as u8, b as u8]));
        }
    }

    new_image.into()
}