use rust_swig::{DotNetConfig, LanguageConfig};

use std::{env, path::Path, path::PathBuf};

fn main() {
    env_logger::init();
    let out_dir = env::var("OUT_DIR").unwrap();
    let in_src = Path::new("src").join("glue.rs.in");
    let out_src = Path::new(&out_dir).join("glue.rs");
    let lib_path = PathBuf::from(&out_dir);

    let config = DotNetConfig::new("etebase_csharp".to_owned(), lib_path);
    let swig_gen = rust_swig::Generator::new(LanguageConfig::DotNetConfig(config))
        .rustfmt_bindings(true);
    swig_gen.expand("csharp bindings", &in_src, &out_src);
    println!("cargo:rerun-if-changed={}", in_src.display());
}
