use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

fn main() {
    let articles_dir = Path::new("src/articles");
    let out_file = articles_dir.join("generated.rs");

    println!("cargo:rerun-if-changed=src/articles");

    let mut entries = fs::read_dir(articles_dir)
        .expect("Failed to read articles directory")
        .filter_map(Result::ok)
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                == Some("md")
        })
        .collect::<Vec<_>>();

    entries.sort_by_key(|e| e.path());

    let mut file = File::create(&out_file)
        .expect("Failed to create generated.rs");

    writeln!(
        file,
        "// AUTO-GENERATED — DO NOT EDIT\n\
         use super::Article;\n\
         use super::FrontMatter;\n"
    )
    .unwrap();

  //  writeln!(file, "pub fn generated_articles() -> Vec<Article> {{").unwrap();
  //  writeln!(file, "    let mut articles = Vec::new();").unwrap();
//
  //  for entry in entries {
  //      let path = entry.path();
  //      let stem = path.file_stem().unwrap().to_string_lossy();
  //      let rel = path.strip_prefix("src/").unwrap();
  //  }

    writeln!(file, "    articles").unwrap();
  //  writeln!(file, "}").unwrap();
}
