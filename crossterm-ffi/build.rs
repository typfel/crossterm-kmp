use camino::Utf8Path;

fn main() {
    let udl_file_path = "./src/crossterm.udl";
    let udl_file = Utf8Path::new(&udl_file_path);
    uniffi::generate_scaffolding(udl_file).unwrap();
}
