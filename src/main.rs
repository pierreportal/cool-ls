use chrono::{DateTime, Utc};
use clap::Parser;
use nerd_font_symbols::md::{MD_FILE, MD_FOLDER};
use owo_colors::OwoColorize;
use std::{fs, path::Path, path::PathBuf};
use tabled::settings::object::{Columns, Rows};
use tabled::settings::{Alignment, Color};
use tabled::{Table, Tabled, settings::Style};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    path: Option<PathBuf>,
    #[arg(short)]
    a: bool,
}

#[derive(Debug, Tabled)]
struct FileEntry {
    #[tabled{rename="Name"}]
    name: String,
    #[tabled{rename="Size"}]
    len_bytes: String,
    #[tabled{rename="Last updated"}]
    modified: String,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or(PathBuf::from("."));
    let show_hidden = cli.a;

    match fs::exists(&path) {
        Ok(path_exists) => {
            if path_exists {
                let files = get_files(&path, show_hidden);
                draw_table(files);
            } else {
                println!("{}", "Path does not exist.".red())
            }
        }
        Err(_) => println!("{}", "Error reading directory".red()),
    }
}

fn get_files(path: &Path, show_hidden: bool) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                map_entry_data(file, &mut data, show_hidden);
            } else {
                continue;
            }
        }
    }
    data
}

fn format_entry_name(file: fs::DirEntry, is_dir: bool) -> (String, bool) {
    let entry_name = file.file_name().into_string().unwrap_or("Unknown".into());
    let file_is_hidden = entry_name.starts_with(".");
    let entry_type_icon = if is_dir { MD_FOLDER } else { MD_FILE };
    let formated_entry_name = format!("{entry_type_icon} {entry_name}");
    (formated_entry_name, file_is_hidden)
}

fn map_entry_data(file: fs::DirEntry, data: &mut Vec<FileEntry>, show_hidden: bool) {
    if let Ok(metadata) = fs::metadata(file.path()) {
        let (formated_entry_name, is_hidden) = format_entry_name(file, metadata.is_dir());
        if !is_hidden || show_hidden {
            data.push(FileEntry {
                name: formated_entry_name,
                len_bytes: format!("{} bytes", metadata.len()),
                modified: if let Ok(modified) = metadata.modified() {
                    let date: DateTime<Utc> = modified.into();
                    let formated_date = format!("{}", date.format("%a %b %e %Y"));
                    formated_date
                } else {
                    "Unknown".to_string()
                },
            });
        }
    }
}

fn draw_table(files: Vec<FileEntry>) {
    let mut table = Table::new(files);
    table.with(Style::rounded());
    table.modify(Columns::one(0), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::one(1), Color::FG_BRIGHT_MAGENTA);
    table.modify(Columns::one(2), Color::FG_YELLOW);
    table.modify(Columns::one(1), Alignment::right());
    table.modify(Rows::one(0), Color::FG_BRIGHT_WHITE);
    table.modify(Rows::one(0), Alignment::center());
    println!("{}", table);
}
