use arboard::{Clipboard};
use clap::Parser;
use std::fs;
use tabled::{Table, Tabled, settings::{Style}};

#[derive(Tabled)]
struct BoardTable {
    idx: usize,
    content: String,
}

struct Board {
    clipboard : Clipboard,
    file : String,
    content : Vec<String>,
}

impl Board {
    pub fn new() -> Self {
        let mut instance = Self { clipboard : Clipboard::new().unwrap(), file: "clips.json".to_string(), content: Vec::new() };
        instance.load_from_file();
        instance
    }

    pub fn add(&mut self, text: String) {
        if !self.content.contains(&text) {
            self.content.push(text);
        }
    }

    pub fn remove(&mut self, idx: usize) -> Option<String> {
        if idx < self.content.len() {
            return Some(self.content.remove(idx));
        }
        None
    }

    pub fn set_clipboard(&mut self, idx : usize) {
        if idx < self.content.len() {
            self.clipboard.set().text(&self.content[idx]);
        }
    }

    pub fn copy_clipboard_to_board(&mut self) {
        let clipboard_content = self.clipboard.get_text().unwrap();
        self.add(clipboard_content);
    }

    pub fn show(&self) {
        // let mut table = Table::new
        let mut data_format = Vec::<BoardTable>::new();

        for (idx, content) in self.content.iter().enumerate() {
            data_format.push(BoardTable { idx, content: content.to_string() });
        }

        let mut table = Table::new(data_format);
        table.with(Style::modern());
        println!("{}", table);
    }

    pub fn load_from_file(&mut self) {
        if !fs::exists(&self.file).unwrap_or(false) { return };
        let json_content = fs::read_to_string(&self.file).unwrap();
        let json_content = serde_json::from_str::<Vec<String>>(&json_content).unwrap();
        self.content = json_content;
    }

    pub fn save_to_file(&self) {
        let clipboard_content = serde_json::to_string(&self.content).unwrap();
        fs::write(&self.file, clipboard_content);
    }

}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, conflicts_with = "copy")]
    remove: Option<usize>,

    #[arg(long)]
    show: bool,

    #[arg(short, long, conflicts_with = "remove")]
    copy: Option<usize>,

    #[arg(short, long)]
    save: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut board = Board::new();

    if std::env::args().into_iter().len() <= 1 {
        board.show();
        return
    }

    if cli.save {
        board.copy_clipboard_to_board();
    }

    if cli.show {
        board.show();
    }

    if let Some(idx) = cli.copy {
        board.set_clipboard(idx);
    }

    board.save_to_file();
}
