use globmatch::{self, IterAll};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn finder(rootpath: &str, words: &str, extention: &str) -> IterAll<PathBuf> {
    // words の　一番目の単語をターゲットワードに設定
    let target = format!(
        "./**/*{}*.{}",
        words.split(" ").collect::<Vec<&str>>()[0],
        extention
    );
    let rootpath = Path::new(rootpath).to_str().unwrap();

    let files = globmatch::Builder::new(&target)
        .case_sensitive(false)
        .build(rootpath)
        .unwrap()
        .into_iter();

    files
}

pub fn opendir(fpath: &PathBuf) -> Result<(), std::io::Error> {
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

pub fn andsearch(path: PathBuf, words: &str) -> bool {
    if !words.contains(" ") {
        return true;
    }
    for wd in words.split_whitespace() {
        if wd != "*"
            && !(path
                .to_str()
                .unwrap()
                .to_lowercase()
                .contains(&wd.to_lowercase()))
        {
            return false;
        }
    }
    true
}
