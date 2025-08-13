use magick_rust::MagickWand;
use std::fs::File;
use std::path::PathBuf;

pub struct Fixture {
    filename: &'static str,
    width: usize,
    height: usize,
}

impl Fixture {
    const fn new(filename: &'static str, width: usize, height: usize) -> Self {
        Self {
            filename,
            width,
            height,
        }
    }

    fn path(&self) -> PathBuf {
        PathBuf::from("tests/fixtures").join(self.filename)
    }

    pub fn file(&self) -> File {
        File::open(self.path())
            .unwrap_or_else(|e| panic!("failed to open file {:?}: {}", self.path(), e))
    }

    fn path_to_string(&self) -> String {
        self.path()
            .to_str()
            .expect("Failed to form path")
            .to_owned()
    }

    pub fn read_image(&self, wand: &MagickWand) {
        wand.read_image(&self.path_to_string())
            .unwrap_or_else(|e| panic!("Failed to read image {}: {}", self.filename, e));
    }

    pub fn assert_width(&self, wand: &MagickWand) {
        assert_eq!(
            self.width,
            wand.get_image_width(),
            "width mismatch in image {}",
            self.filename
        );
    }

    pub fn assert_height(&self, wand: &MagickWand) {
        assert_eq!(
            self.height,
            wand.get_image_height(),
            "height mismatch in image {}",
            self.filename
        );
    }
}

pub const IMG_5745_JPG: Fixture = Fixture::new("IMG_5745.JPG", 512, 384);
pub const IMG_5745_ROTL_JPG: Fixture = Fixture::new("IMG_5745_rotl.JPG", 384, 512);
pub const RUST_PNG: Fixture = Fixture::new("rust.png", 240, 240);
pub const RUST_GIF: Fixture = Fixture::new("rust.gif", 80, 76);
pub const RUST_SVG: Fixture = Fixture::new("rust.svg", 144, 144);

pub const ALL_FIXTURES: [Fixture; 5] = [
    IMG_5745_JPG,
    IMG_5745_ROTL_JPG,
    RUST_PNG,
    RUST_GIF,
    RUST_SVG,
];
