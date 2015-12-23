// build.rs

fn main() {
    // For the sake of easily building and testing on Mac, include the path
    // to MagickWand. Chances are MagickWand is in /usr/local/lib, or
    // somewhere else that rustc can find it.
    println!("cargo:rustc-link-search=native=/usr/local/lib");
}
