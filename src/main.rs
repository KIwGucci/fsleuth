mod filefinder;
use anyhow::{bail, Result};
use crossterm::{execute, terminal};
use rustyline::Editor;
use std::{
    io::{stdout, BufWriter, Write},
    path::PathBuf,
};

const DISPLAY_LIMIT:usize = 100;

fn main() -> Result<()> {
    let mut my_readline = Editor::<()>::new()?;

    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;

    println!("File Finder.\nInput help to see func");

    let mut app = FileFinder::new();

    app.set_extension(&mut my_readline)?;

    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;

    app.manage_token(&mut my_readline)?;

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

    fn search(&mut self) -> Result<()> {
        let mut output_buffer = BufWriter::new(stdout());
        self.stack_vec = filefinder::item_search(".", &self.extention, &self.searchword);

        // 表示する項目数の制限
        let limitnum = self.stack_vec.len().min(DISPLAY_LIMIT);

        for it in self.stack_vec[..limitnum].iter() {
            writeln!(output_buffer, "{:?}", it)?;
        }

        writeln!(
            output_buffer,
            "extention: {},word: {} => {} hits",
            self.extention,
            self.searchword,
            self.stack_vec.len()
        )?;

        output_buffer.flush()?;
        Ok(())
    }

    fn set_extension(&mut self, my_readline: &mut Editor<()>) -> Result<()> {
        loop {
            match my_readline.readline("extension >>") {
                Ok(rline) => {
                    if rline.is_empty() {
                        println!("please input search word... '*' is wildcard.");
                        continue;
                    } else {
                        self.extention = rline;
                    }
                }

                Err(e) => {
                    bail!("{e}")
                }
            }
            println!("Extention:{}", self.extention);

            break;
        }
        Ok(())
    }

    fn open_file(&mut self, my_readline: &mut Editor<()>) -> Result<()> {
        let limitnum = 100;

        loop {
            execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
            let mut output_buffer = BufWriter::new(stdout());
            for (i, path) in self.stack_vec.iter().enumerate() {
                if i >= limitnum {
                    writeln!(
                        output_buffer,
                        "There are more than {limitnum} applicable items"
                    )?;
                    break;
                }

                writeln!(output_buffer, "{}:{:?}", i, path)?;
            }

            output_buffer.flush()?;

            match my_readline.readline("select number >>") {
                Ok(rline) => match rline.as_str() {
                    "q" | "quit" | "@q" | "@quit" => {
                        break;
                    }

                    _ => match rline.parse::<usize>() {
                        Ok(n) if n < self.stack_vec.len() => {
                            filefinder::opendir(&self.stack_vec[n])?;
                        }
                        Ok(_) => {
                            println!("Wrong number.");
                        }

                        Err(e) => println!("{e}"),
                    },
                },

                Err(e) => {
                    bail!("{e}");
                }
            }
        }
        Ok(())
    }

    fn manage_token(&mut self, my_readline: &mut Editor<()>) -> Result<()> {
        let mut output_buffer = BufWriter::new(stdout());

        loop {
            writeln!(
                output_buffer,
                "@ext << change extention, @open << open file, @q or @quit << exit"
            )?;
            output_buffer.flush()?;

            let in_token: String;
            let readline = my_readline.readline(format!("ext:{} =>>", self.extention).as_str());

            execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;

            match readline {
                Ok(rline) => in_token = rline.trim().to_string(),
                Err(e) => {
                    bail!("{e}")
                }
            }

            match in_token.as_str() {
                "@quit" | "@q" => break,

                "@ext" | "@e" | "@ex" => {
                    self.set_extension(my_readline)?;
                }
                "@open" | "@o" | "@op" => {
                    self.open_file(my_readline)?;
                }
                _ => {
                    self.searchword = in_token.to_string();
                    self.search()?;
                }
            }
        }
        Ok(())
    }
}
