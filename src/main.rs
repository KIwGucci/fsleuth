mod filefinder;
use anyhow::{bail, Result};
use crossterm::{execute, terminal};
use globmatch::IterAll;
use rustyline::Editor;
use std::{
    io::{stdout, BufWriter, Write},
    path::PathBuf,
};

fn main() -> Result<()> {
    execute!(stdout(), terminal::EnterAlternateScreen)?;

    let mut my_readline = Editor::<()>::new()?;

    println!("File Finder.\nInput help to see func");
    let mut app = FileFinder::new();
    let mut output_buffer = BufWriter::new(stdout());

    loop {

        let readline = my_readline.readline("extension >>");

        match readline {
            Ok(rline) => {
                app.extention = rline;
                if app.extention == "" {
                    writeln!(
                        output_buffer,
                        "please input search word... '*' is wildcard."
                    )?;

                    output_buffer.flush()?;
                    continue;
                }
            }

            Err(e) => {
                bail!("{e}")
            }
        }
        writeln!(output_buffer, "Extention:{}", app.extention)?;
        output_buffer.flush()?;

        break;
    }

    loop {
        writeln!(
            output_buffer,
            "@ext << change extention, @open << open file, @q or @quit << exit"
        )?;
        output_buffer.flush()?;

        let in_token: String;
        let readline = my_readline.readline(format!("ext:{} =>>", app.extention).as_str());

        match readline {
            Ok(rline) => in_token = rline.trim().to_string(),
            Err(e) => {
                bail!("{e}")
            }
        }

        match in_token.as_str() {
            "@quit" | "@q" => break,
            "@ext" | "@e" | "@ex" => {
                let readline = my_readline.readline("extension >>");
                match readline {
                    Ok(rline) => app.extention = rline,
                    Err(e) => {
                        bail!("{e}");
                    }
                }
                writeln!(output_buffer, "Extention:{}", app.extention)?;
                output_buffer.flush()?;
                continue;
            }
            "@open" | "@o" | "@op" => {
                loop {
                    let limitnum = 100;
                    for (i, path) in app.stack_vec.iter().enumerate() {
                        if i >= limitnum {
                            writeln!(
                                output_buffer,
                                "There are more than {limitnum} applicable items"
                            )?;
                            break;
                        } else {
                            writeln!(output_buffer, "{}:{:?}", i, path)?;
                        }
                    }
                    output_buffer.flush()?;
                    let readline = my_readline.readline("select number >>");

                    match readline {
                        Ok(rline) => {
                            if let "q" | "quit" | "@q" | "@quit" = rline.as_str() {
                                break;
                            } else {
                                let num = rline.parse::<usize>();
                                match num {
                                    Ok(n) => {
                                        if n < app.stack_vec.len() {
                                            filefinder::opendir(&app.stack_vec[n])?;
                                        } else {
                                            println!("Wrong number.");
                                        }
                                    }
                                    Err(e) => bail!("{e}"),
                                }
                            }
                        }
                        Err(e) => {
                            bail!("{e}");
                        }
                    }
                }
                continue;
            }
            _ => app.searchword = in_token.to_string(),
        }

        if let Some(items) = app.search() {
            app.stack_vec = Vec::new();
            // 該当項目のcounterとスタックする上限
            let mut counter = 0;
            let stack_limit = 100;

            // 該当する項目のカウントと要素のスタック
            for item in items.into_iter() {
                let it = match item.to_owned() {
                    Ok(fs) => fs,
                    Err(e) => {
                        bail!("{e}")
                    }
                };

                if filefinder::andsearch(it, &app.searchword) {
                    if app.stack_vec.len() < stack_limit {
                        app.stack_vec.push(item.unwrap());
                    }
                    counter += 1;
                }
            }

            // 表示する項目数の制限
            let limitnum = app.stack_vec.len().min(stack_limit);

            for it in app.stack_vec[..limitnum].iter() {
                writeln!(output_buffer, "{:?}", it)?;
            }

            writeln!(
                output_buffer,
                "extention: {},word: {} => {} hits",
                app.extention, app.searchword, counter
            )?;
        } else {
            writeln!(output_buffer, "some thing wrong")?;
        }
        clearscreen()?;
        output_buffer.flush()?;
    }

    execute!(stdout(), terminal::LeaveAlternateScreen)?;
    Ok(())
}

struct FileFinder {
    stack_vec: Vec<PathBuf>,
    extention: String,
    searchword: String,
}
impl FileFinder {
    fn new() -> Self {
        FileFinder {
            stack_vec: Vec::new(),
            extention: "".to_string(),
            searchword: "".to_string(),
        }
    }
    fn search(&self) -> Option<IterAll<PathBuf>> {
        let result = filefinder::finder(".", &self.searchword, &self.extention);
        Some(result)
    }
}

fn clearscreen() -> Result<()> {
    execute!(stdout(), terminal::Clear(terminal::ClearType::Purge))?;
    Ok(())
}
