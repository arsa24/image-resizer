use clap::{Arg, Command};
use image::{
    imageops::resize, GenericImageView, ImageFormat as ImageFormater, ImageReader as imgreader,
};
use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
    let matches = Command::new("Image Resizer")
        .version("1.0")
        .author("Arsa")
        .arg(
            Arg::new("file")
                .help("Path to the input image file")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("reduce")
                .help("Set the image quality (1-100)")
                .short('r')
                .long("reduce")
                .value_parser(clap::value_parser!(u8).range(1..=100))
                .default_value("50"),
        )
        .arg(
            Arg::new("extension")
                .help("Specify output file extension")
                .short('e')
                .long("extension")
                .default_value("jpg"),
        )
        .arg(
            Arg::new("all_files")
                .help("Process all image files in the directory")
                .short('a')
                .long("all")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("delete")
                .help("Delete the input file after processing")
                .short('d')
                .long("delete")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let process_all = matches.get_flag("all_files");
    let delete = matches.get_flag("delete");
    let extension = matches
        .get_one::<String>("extension")
        .expect("Invalid extension");
    let reduce: u8 = *matches
        .get_one::<u8>("reduce")
        .expect("Reduce value should always be present due to default_value");

    println!(
        "all: {}, delete: {}, extension: {}, reduce: {}%",
        process_all, delete, extension, reduce
    );
    let get_path = get_current_dir()?;

    if process_all {
        let files = get_img_files(&get_path)?;
        for file in files {
            let full_path = format!("{}/{}", get_path, file);
            if img_validation(&full_path) {
                let fname = get_file_name(&full_path).unwrap_or_else(|| "output".to_string());
                resizer_image(&full_path, reduce, &fname, extension);
                if delete {
                    fs::remove_file(full_path).expect("Failed to remove file");
                }
            }
        }
    }
    Ok(())
}

fn get_current_dir() -> io::Result<String> {
    let current_dir = env::current_dir()?;
    current_dir
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to convert path to string"))
}

fn get_img_files(path: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(path)?;
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        if let Some(name) = file_name.to_str() {
            files.push(name.to_string());
        }
    }
    Ok(files)
}

fn resizer_image(path_img: &str, reduce: u8, output: &str, extension: &str) {
    let outputfile = format!("{}.{}", output, extension);

    let img = imgreader::open(path_img)
        .expect("Failer to open image file")
        .decode()
        .expect("Failer to decode image");
    let (default_w, default_h) = img.dimensions();

    let resized = resize(
        &img,
        reduce_count(reduce, default_w),
        reduce_count(reduce, default_h),
        image::imageops::FilterType::Lanczos3,
    );
    let output_format = ImageFormater::from_path(&outputfile).expect("Invalid output format");
    resized
        .save_with_format(&outputfile, output_format)
        .expect("Failed to save output file");
    println!("Image resized and saved to {}", outputfile);
}

fn reduce_count(r: u8, x: u32) -> u32 {
    let p = (x * r as u32) / 100;
    return p;
}

fn img_validation(path: &String) -> bool {
    let valid_extensions = ["jpg", "jpeg", "png", "gif", "bmp", "webp"];
    if let Some(extension) = Path::new(path).extension() {
        if let Some(ext_str) = extension.to_str() {
            return valid_extensions.contains(&ext_str.to_lowercase().as_str());
        }
    }
    false
}

fn get_file_name(path: &str) -> Option<String> {
    let p = Path::new(path);
    if let Some(file_name) = p.file_stem() {
        if let Some(name_str) = file_name.to_str() {
            return Some(name_str.to_string());
        }
    }
    None
}
