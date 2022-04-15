
use serde::{Serialize, Deserialize};
use structopt::StructOpt;
use crate::{ImageFormat, AlignmentMode};

#[derive(Debug, Clone, StructOpt, Serialize, Deserialize)]
#[structopt(name = "")]
pub struct Opt {

    #[structopt(long)]
    #[serde(skip_serializing, default)]
    pub help: bool,

    #[structopt(long)]
    #[serde(skip_serializing, default)]
    pub version: bool,

    #[structopt(short, long)]
    pub horizontal: bool,

    #[structopt(short, long)]
    pub vertical: bool,

    #[structopt(long, default_value="0")]
    pub maxd: usize,

    #[structopt(long, default_value="0")]
    pub maxw: usize,

    #[structopt(long, default_value="0")]
    pub maxh: usize,

    #[structopt(short, long)]
    pub reverse: bool,

    #[structopt(long)]
    pub jpeg: bool,

    #[structopt(long)]
    pub png: bool,

    #[structopt(long)]
    pub gif: bool,

    #[structopt(long)]
    pub bmp: bool,

    #[structopt(long, default_value="100")]
    pub quality: usize,

    #[structopt(long)]
    pub ascalpha: bool,

    #[structopt(long)]
    pub descalpha: bool,

    #[structopt(required_unless_one = &["help", "version"])]
    pub number_of_files: Option<usize>
}

impl Default for Opt {
    fn default() -> Self {
        Opt {
            help: false,
            version: false,
            horizontal: false,
            vertical: false,
            maxd: 0,
            maxw: 0,
            maxh: 0,
            reverse: false,
            jpeg: false,
            png: false,
            gif: false,
            bmp: false,
            quality: 100,
            ascalpha: false,
            descalpha: false,
            number_of_files: None
        }
    }
}

impl Opt {
    pub fn check_for_basic_errors(&self) -> Option<&'static str> {

        // Verify not requesting both ascending and descending alphabetical order
        if self.ascalpha && self.descalpha {
            return Some("If selecting files based on alphabetical order, choose ascending or descending, not both.");
        }

        // Verify not requesting both horizontal and vertical
        if self.horizontal && self.vertical {
            return Some("Choose either horizontal or vertical (or neither), not both.");
        }

        // Verify not requesting overlapping constraints
        if self.maxd > 0 && self.maxw > 0 {
            return Some("If using maxd, do not specify maxw as well.");
        }
        if self.maxd > 0 && self.maxh > 0 {
            return Some("If using maxd, do not specify maxh as well.");
        }

        // Choose one format only, or none at all
        let format_flag_set: [bool; 4] = [self.jpeg, self.png, self.gif, self.bmp];
        let format_flag_count: i32 = format_flag_set.iter().map(|&f| { if f { 1 } else { 0 } }).sum();
        if format_flag_count > 1 {
            return Some("You cannot specify more than one of image types JPEG, PNG, GIF and BMP.");
        }

        // Verify quality setting is within the appropriate range, and is only used for JPEG
        if self.quality == 0 || self.quality > 100 {
            return Some("The quality setting must be in the range of 1 to 100 inclusive.");
        }
        if self.quality != 100 && !self.jpeg && format_flag_count > 0 {
            return Some("The quality setting can only be used for JPEG output.");
        }

        // Verify a sensible number was given
        let number_of_files = match self.number_of_files {
            Some(num) => num,
            _ => return Some("You did not provide number_of_files and StructOpt did not catch this error")
        };
        if number_of_files == 0 {
            return Some("The number of images to stitch must be at least 1.");
        }

        None
    }

    pub fn prepare_for_use(&mut self) {
        if self.maxd > 0 {
            self.maxw = self.maxd;
            self.maxh = self.maxd;
        }
    }

    pub fn get_requested_image_format(&self) -> ImageFormat {
        if self.jpeg {
            ImageFormat::Jpeg
        } else if self.png {
            ImageFormat::Png
        } else if self.gif {
            ImageFormat::Gif
        } else if self.bmp {
            ImageFormat::Bmp
        } else {
            ImageFormat::Unspecified
        }
    }

    pub fn get_alignment(&self) -> AlignmentMode {
        match (self.horizontal, self.vertical) {
            (true, false) => AlignmentMode::Horizontal,
            (false, true) => AlignmentMode::Vertical,
            _ => AlignmentMode::Grid
        }
    }
}

#[cfg(test)]
mod test {
    use super::Opt;

    #[test]
    fn serializes_okay() {
        let expected = Opt {
            horizontal: true,
            maxw: 120,
            png: true,
            number_of_files: None,
            ..Opt::default()
        };
        let from_json = "{ \
         \"help\": false, \
         \"version\": false, \
         \"horizontal\": true, \
         \"vertical\": false, \
         \"maxd\": 0, \
         \"maxw\": 120, \
         \"maxh\": 0, \
         \"reverse\": false, \
         \"jpeg\": false, \
         \"png\": true, \
         \"gif\": false, \
         \"bmp\": false, \
         \"quality\": 100, \
         \"ascalpha\": false, \
         \"descalpha\": false \
         }";
        let result: Opt = serde_json::from_str(from_json).unwrap();
        assert_eq!(format!("{:?}", expected), format!("{:?}", result));
    }
}
