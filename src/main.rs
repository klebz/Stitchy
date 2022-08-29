pub mod enums;
pub mod files;
pub mod image_set;
pub mod options;
pub mod print;
pub mod run;
pub mod profiles;

use enums::{AlignmentMode, ImageFormat};
use files::ImageFiles;
use image_set::ImageSet;
use structopt::StructOpt;

fn main() {

    // Get command line args, check for flags that merely print to the console and exit
    let mut opt = options::Opt::from_args();
    if opt.help {
        print::help();
        return;
    }
    if opt.version {
        print::version();
        return;
    }
    if opt.printdefaults {
        print::defaults();
        return;
    }

    // Save options if requested, or try to load stored options otherwise
    if opt.setdefaults {

        if let Some(error) = opt.check_for_basic_errors() {
            println!("Cannot save settings. {}", error);
            return;
        }
        if let Some(json) = opt.serialise() {
            profiles::Profile::main().write_string(json);
        }

    } else if opt.cleardefaults {

        profiles::Profile::main().delete();

    } else if let Some(json) = profiles::Profile::main().to_string() {

        if let Some(profile_opt) = options::Opt::deserialise(&json) {
            opt = opt.mix_in(profile_opt);
        }
    }

    // Check conditions where the user did not request a number of files, but this is allowed
    // because some operations on the defaults file does not require that files are processed now
    if opt.number_of_files.is_none() && (opt.setdefaults || opt.cleardefaults) {
        return;
    }

    // Perform simple validation
    if let Some(error) = opt.check_for_basic_errors() {
        println!("{}", error);
        return;
    }

    // Ensure some number of files was provided
    if let Some(error) = opt.check_number_of_files_provided() {
        println!("{}", error);
        return;
    }

    // Pre-use preparations
    opt.prepare_for_use();

    // Call function to do all the file processing, print final messages here
    match stitch_images(opt) {
        Ok(msg) => println!("{}", msg),
        Err(msg) => println!("{}", msg)
    }
}
