use serde::Deserialize;
use std::error::Error;
use std::fs::{self, File};
use std::io::copy;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(String),        // filename
    ApiError(String),
    NetworkError(String),
    FileError(String),
}

fn guess_extension(url: &str) -> &str {
    if let Some(ext) = url.rsplit('.').next() {
        let ext = ext.split(|c: char| c == '?' || c == '#').next().unwrap_or(ext);
        match ext.to_ascii_lowercase().as_str() {
            "jpg" | "jpeg" | "png" | "gif" | "webp" => ext,
            _ => "jpg",
        }
    } else {
        "jpg"
    }
}

fn fetch_random_dog_image(i: usize) -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";

    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => {
                        let image_url = dog_image.message;

                        // Download the image itself
                        match ureq::get(&image_url).call() {
                            Ok(img_response) => {
                                if let Err(e) = fs::create_dir_all("images") {
                                    return ApiResult::FileError(format!("Failed to create folder: {}", e));
                                }

                                let ext = guess_extension(&image_url);
                                let filename = format!("images/dog_{}.{}", i, ext);

                                match File::create(&filename) {
                                    Ok(mut file) => {
                                        let mut reader = img_response.into_reader();
                                        if let Err(e) = copy(&mut reader, &mut file) {
                                            return ApiResult::FileError(format!("Write failed: {}", e));
                                        }
                                        ApiResult::Success(filename)
                                    }
                                    Err(e) => ApiResult::FileError(format!("File create failed: {}", e)),
                                }
                            }
                            Err(e) => ApiResult::NetworkError(format!("Image download failed: {}", e)),
                        }
                    }
                    Err(e) => ApiResult::ApiError(format!("JSON parse failed: {}", e)),
                }
            } else {
                ApiResult::ApiError(format!("HTTP error: {}", response.status()))
            }
        }
        Err(e) => ApiResult::NetworkError(format!("Request failed: {}", e)),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Downloader");
    println!("=======================\n");

    for i in 1..=5 {
        println!("Fetching image #{}", i);
        match fetch_random_dog_image(i) {
            ApiResult::Success(filename) => println!("Saved: {}\n", filename),
            ApiResult::ApiError(e) => println!("API Error: {}\n", e),
            ApiResult::NetworkError(e) => println!("Network Error: {}\n", e),
            ApiResult::FileError(e) => println!("File Error: {}\n", e),
        }
    }

    Ok(())
}
