use windres::Build;

fn main() {
    Build::new().compile("booter.rc").unwrap();
}
