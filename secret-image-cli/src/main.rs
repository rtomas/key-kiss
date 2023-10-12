use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use rand::Rng;
use rusttype::{Font, Scale};
use sha256::digest;
use std::fs;
use std::io;
use std::process::exit;

fn main() {
    std::process::Command::new("clear").status().unwrap();
    println!("Welcome to 'key-kiss' (key keep it safe/secure) CLI");
    println!(" ");
    println!(" --- don't forget to delete the output image after using it ---");
    println!(" ");

    let magic_string = get_cli_string("Enter your password (min 8 caracters): ");

    let vec_hex: Vec<String> = (0..64)
        .map(|i| get_letters(magic_string.clone(), i))
        .collect();

    // delete the all files, so there is no confusions with others
    delete_png_files(".").expect("Error deleting png files");

    create_image_from_string(vec_hex);
    exit(0);
}

fn create_image_from_string(vec: Vec<String>) {
    println!("Creating image with v1.0 ...",);
    let mut image = RgbImage::new(7700, 2600);
    let font = Vec::from(include_bytes!("./font/JetBrainsMono-Regular.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let height = 120.4;
    let scale = Scale {
        x: height * 2.0,
        y: height,
    };

    //background to white
    for pixel in image.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    let mut p_x;

    // this is because i cannot print vertical a string.
    for p_y in 0..16 {
        p_x = 0;

        for line in vec.clone() {
            let char_str = &line[p_y..p_y + 1].to_string();

            let val_x = p_x * 120 + (p_x / 4); // to adjunts to the "white page" squares

            draw_text_mut(
                &mut image,
                Rgb([0u8, 0u8, 0u8]),
                val_x,
                (p_y * 160) as i32,
                scale,
                &font,
                char_str,
            );
            p_x += 1;
        }
    }

    image.save(generate_random_name(5) + ".png").unwrap();
}

fn get_letters(magic_string: String, i: u32) -> String {
    let magic_number = string_to_number(magic_string + &i.to_string());

    let vec_order = order_numbers(magic_number);
    vec_order.iter().map(|x| format!("{:X}", x)).collect()
}

// transfor the string into a determinist number throw a hash function
fn string_to_number(mut magic_string: String) -> u32 {
    let mut magic_number: u32 = 1;
    // sha256 to magic_string
    magic_string = digest(magic_string.as_bytes());

    // convert the string to a number
    let mut i = 1;
    for c in magic_string.chars() {
        i += 1;
        let c = c as u32;
        if c % 2 == 0 {
            magic_number += c * i;
        } else {
            magic_number += c;
        }
    }

    magic_number
}

fn order_numbers(num_x: u32) -> Vec<u32> {
    let mut numbers: Vec<u32> = (0..=15).collect();

    for m in 0..10 {
        for i in 0..15 {
            let mut j = (i * m * num_x as usize) % 16;
            if i == j {
                j = (i + m + num_x as usize) % 16;
            }
            numbers.swap(i, j);
        }
    }

    numbers
}

fn get_cli_string(message: &str) -> String {
    print!("{}", message);
    flush();
    let mut key = String::new();
    std::io::stdin()
        .read_line(&mut key)
        .expect("Failed to read line");
    return key.trim().parse().expect("Please type a string");
}

// random name for the image
fn generate_random_name(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();
    let name: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    name
}

fn delete_png_files(directory_path: &str) -> io::Result<()> {
    // Iterate over the entries in the directory
    for entry in fs::read_dir(directory_path)? {
        let entry = entry?;
        let file_path = entry.path();

        // Check if the entry is a file and ends with ".png"
        if file_path.is_file() && file_path.extension() == Some("png".as_ref()) {
            // Attempt to delete the file
            fs::remove_file(&file_path)?;
            println!("Deleted: {:?}", file_path);
        }
    }

    Ok(())
}

fn flush() {
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}
