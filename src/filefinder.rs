extern crate globmatch;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn item_search(rootpath: &str, extention: &str, searchword: &str) -> Vec<PathBuf> {
    // words の　一番目の単語をターゲットワードに設定
    let target = format!("./**/*.{}", extention);

    let rootpath = Path::new(rootpath);

    let builder = globmatch::Builder::new(&target)
        .case_sensitive(false)
        .build(rootpath)
        .expect("Could not build globmatcher");

    let paths = builder.into_iter().filter_map(|x| x.ok());

    paths.filter(|f| is_contain(f, searchword)).collect()
}

pub fn opendir(fpath: &Path) -> Result<(), std::io::Error> {
    // 対象ファイルのフォルダをファインダーで開く
    #[cfg(target_os = "macos")]
    Command::new("open").arg(fpath.parent().unwrap()).spawn()?;

    #[cfg(target_os = "windows")]
    Command::new("explorer")
        .arg(fpath.parent().unwrap())
        .spawn()?;

    #[cfg(target_os = "linux")]
    Command::new("xdg-open")
        .arg(fpath.parent().unwrap())
        .spawn()?;

    Ok(())
}

fn is_contain(path: &Path, words: &str) -> bool {

    for wd in words.split_whitespace() {
        if wd != "*"
            && !(path
                .to_string_lossy()
                .to_lowercase()
                .contains(&wd.to_lowercase()))
        {
            return false;
        }
    }

    true
}
