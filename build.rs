use protoc_rust::Customize;
use std::path::Path;

fn main() {
    let a = Path::new("src/proto").join("A").join("a.proto");
    let b = Path::new("src/proto").join("B").join("b.proto");
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src",
        input: &[a.to_str().unwrap(),b.to_str().unwrap()],
        includes: &["src/proto"],
        customize: Customize {
            ..Default::default()
        },
    }).expect("protoc");
}