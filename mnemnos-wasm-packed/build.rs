use std::{env, fs, os::unix::process::CommandExt, path::Path, process::Command};

// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=../src/**");
    // TODO: figure out how to bundle trunk instead of relying on having it locally

    let wasm_build_dir = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("..")
        .join("mnemnos-wasm");

    // if cfg!(target_os = "windows") {
    //     panic!("windows not supported")
    // } else {
    //     // todo: panic if trunk not available
    //     Command::new("trunk")
    //         .arg("build") // todo set -- release if set via cargo
    //         .current_dir(&wasm_build_dir)
    //         .spawn()
    //         .expect("failed to execute process")
    // };

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("gen.rs");

    let mut files = Vec::new();

    for path in fs::read_dir(Path::new(&wasm_build_dir).join("dist")).unwrap() {
        let path = path.unwrap();
        let mime_guess = mime_guess::from_path(path.path())
                .first_raw()
                .map( |s| s.to_string())
                .unwrap_or(mime_guess::mime::APPLICATION_OCTET_STREAM.to_string())
                // .map(HeaderValue::from_static)
                // .unwrap_or_else(|| {
                //     HeaderValue::from_str(.as_ref()).unwrap()
                // })

        ;
        files.push((path.file_name().into_string().unwrap(), mime_guess));
        fs::copy(path.path(), Path::new(&out_dir).join(path.file_name())).unwrap();
    }

    let routes: String = files.into_iter()
        .map(|(path, mime)| if path == "index.html" {
            format!(".route(\"/\", get(|| async {{ mk(include_bytes!(\"{path}\"), \"{}\") }} ))", mime)
        } else {
            format!(".route(\"/{path}\", get(|| async {{ mk(include_bytes!(\"{path}\"), \"{}\") }} ))", mime)
        })
        .collect();

    // let routes: String = files.into_iter().map(|s| format!("\"{s}\" => include_bytes!(\"{s}\"),\n")).collect();

    fs::write(
        &dest_path,
        format!(
            r#"

            use axum::*;
            use axum::response::*;
            use axum::routing::*;
            use axum::body::*;

        fn mk(b: &'static [u8], m: &'static str) -> impl IntoResponse {{
            Response::builder().header("Content-Type", m).body(Body::from(b)).unwrap()
        }}

        pub fn routes() -> axum::Router {{
            axum::Router::new(){routes}
        }}

        "#
        ),
    )
    .unwrap();
}
