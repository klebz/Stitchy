
pub fn help() {
    println!("Stitchy v{} by {}", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
    println!("Collects a number of image files in the current directory and stitches them into");
    println!("a single file.");
    println!();
    println!("Basic usage:");
    println!("  stitchy n");
    println!("  where n is the number of images to use. There must be at least that many in the");
    println!("  current directory. By default, The most recent images available will be used.");
    println!();
    println!("Supported flags:");
    println!("  --help            Print this help");
    println!("  --version         Print the installed version number");
    println!("  --ascalpha        Select first files based on ascending alphabetical order");
    println!("  --descalpha       Select first files based on descending alphabetical order");
    println!("  --horizontal, -h  Force stitching across a single row only");
    println!("  --vertical, -v    Force stitching down a single column only");
    println!("  --maxw=n          Limit output width to n pixels at most");
    println!("  --maxh=n          Limit output height to n pixels at most");
    println!("  --maxd=n          Limit output width and height to n pixels at most");
    println!("  --reverse, -r     Stitch file in reverse chronological order");
    println!("  --quality=n       Set the output quality from 1 to 100, defaults to 100");
    println!("  --jpeg            Output as JPEG");
    println!("  --png             Output as PNG");
    println!("  --gif             Output as GIF");
    println!("  --bmp             Output as BMP");
    println!("                    Note: default format matches sources, or JPEG where source formats vary")
}

pub fn version() {
    println!("Stitchy version {}", env!("CARGO_PKG_VERSION"));
    println!("Authored by {}", env!("CARGO_PKG_AUTHORS"));
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
}
