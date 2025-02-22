use enums::{AlignmentMode, ImageFormat};
use files::ImageFiles;
use image_set::ImageSet;
use structopt::StructOpt;

/// Runs Stitchy using the supplied options. The options should have been checked for basic errors
/// and prepared for use before calling this function.
pub fn stitch_images(opt: options::Opt) -> Result<String, String> {

    // Determine the list of files to use as input, and from those, determine the output path
    let image_sources = ImageFiles::from_directory(vec!())?
        .sort_and_truncate_by(&opt)?;
    let output_format = image_sources.determine_output_format(&opt)?;
    let output_file_path = image_sources.next_available_output(&opt)?;

    // Open the image files and process them to make the output image
    let images = image_sources.into_image_contents()?;
    let output = ImageSet::new(images, &opt)
        .stitch()?;

    // Write the output file, returning a success message or an error message
    files::write_image_to_file(output, &output_file_path, output_format, opt.quality)?;
    let output_string = match files::size_of_file(&output_file_path) {
        Ok(size_string) => format!(
            "Created file: {:?}, {}", output_file_path.file_name().unwrap(), size_string
        ),
        Err(_) => format!(
            "Created file: {:?}", output_file_path.file_name().unwrap()
        )
    };
    Ok(output_string)
}
