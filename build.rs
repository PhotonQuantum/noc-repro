fn main() {
    for file in glob::glob("src/*.rs").unwrap() {
        eprintln!("{}", file.unwrap().display());
    }
}