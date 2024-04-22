mod image;

use image::{
    load_image_file, 
    apply_grayscale_filter, 
    save_image_file, 
    apply_invert_filter, 
    apply_sepia_filter,
    apply_threshold_filter,
    apply_blur_filter,
    detect_edges,
    detect_footballers,
    diff_images
};

fn main() {
    let image_file_name = "football.jpeg";
    let mut image = load_image_file(image_file_name.to_string());
    
    // grayscale filter
    let grayImage = apply_grayscale_filter(&mut image);
    save_image_file(&grayImage, "gray_football.jpeg".to_string());

    // invert filter
    // let invertedImage = apply_invert_filter(&mut image);
    // save_image_file(&invertedImage, "inverted_football.jpeg".to_string());

    // sepia filter
    // let sepiaImage = apply_sepia_filter(&mut image);
    // save_image_file(&sepiaImage, "sepia_football.jpeg".to_string());

    // threshold filter
    // let mut gray_image_for_threshold_50 = apply_grayscale_filter(&mut image);
    // let threshold_50_image = apply_threshold_filter(&mut gray_image_for_threshold_50, 50);
    // save_image_file(&threshold_50_image, "threshold_50_football.jpeg".to_string());

    // let mut gray_image_for_threshold_100 = apply_grayscale_filter(&mut image);
    // let threshold_100_image = apply_threshold_filter(&mut gray_image_for_threshold_100, 100);
    // save_image_file(&threshold_100_image, "threshold_100_football.jpeg".to_string());

    // let mut gray_image_for_threshold_150 = apply_grayscale_filter(&mut image);
    // let threshold_150_image = apply_threshold_filter(&mut gray_image_for_threshold_150, 150);
    // save_image_file(&threshold_150_image, "threshold_150_football.jpeg".to_string());

    // let mut gray_image_for_threshold_200 = apply_grayscale_filter(&mut image);
    // let threshold_200_image = apply_threshold_filter(&mut gray_image_for_threshold_200, 200);
    // save_image_file(&threshold_200_image, "threshold_200_football.jpeg".to_string());

    // let blur_5_image = apply_blur_filter(&mut image, 5);
    // save_image_file(&blur_5_image, "blur_5_football.jpeg".to_string());

    // let blur_10_image = apply_blur_filter(&mut image, 10);
    // save_image_file(&blur_10_image, "blur_10_football.jpeg".to_string());

    // let edges_image = detect_edges(&mut image);
    // save_image_file(&edges_image, "edges_football.jpeg".to_string());

    // let mut gray_image = apply_grayscale_filter(&mut image);
    // let mut threshold_100_image = apply_threshold_filter(&mut gray_image, 100);
    // let footballers_50_image = detect_footballers(&mut threshold_100_image, 50);
    // save_image_file(&footballers_50_image, "footballers_50_football.jpeg".to_string());

    // let mut gray_image = apply_grayscale_filter(&mut image);
    // let mut threshold_100_image = apply_threshold_filter(&mut gray_image, 100);
    // let footballers_100_image = detect_footballers(&mut threshold_100_image, 100);
    // save_image_file(&footballers_100_image, "footballers_100_football.jpeg".to_string());

    // let diff_image = diff_images(&footballers_50_image, &footballers_100_image);
    // save_image_file(&diff_image, "footballers_50_100_diff.jpeg".to_string());
}