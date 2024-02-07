// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use serde::{Deserialize, Serialize};
use std::io;

use tokio::fs::{create_dir, read_dir, try_exists, write};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

// we must manually implement serde::Serialize
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

const DIR_NAME: &str = "saved_notes";
const DIR_LOC: &str = "..";

#[tauri::command]
async fn save_to_file(filename: String, content: String) -> Result<(), Error> {
    let target_filename = format!("{DIR_LOC}/{DIR_NAME}/{filename}.txt");

    let terget_dir_exists = try_exists(&target_filename).await.is_ok();
    if !terget_dir_exists {
        create_dir(format!("{DIR_LOC}/{DIR_NAME}")).await?;
    }

    write(target_filename, content).await?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Note {
    id: usize,
    title: String,
    content: String,
}

#[tauri::command]
async fn get_notes() -> Result<Vec<Note>, Error> {
    let mut notes = vec![];
    let mut dir_contents = read_dir(format!("{DIR_LOC}/{DIR_NAME}/")).await?;

    while let Some(dir_entry) = dir_contents.next_entry().await? {
        let filepath = dir_entry
            .file_name()
            .to_str()
            .expect("Files should have correct names since only this application creates them.")
            .to_owned();

        let file_contents =
            tokio::fs::read_to_string(format!("{DIR_LOC}/{DIR_NAME}/{filepath}")).await?;

        let lines: Vec<&str> = file_contents.lines().collect();

        notes.push(Note {
            id: 0,
            title: lines[0].into(),
            content: lines[1..].iter().map(ToString::to_string).collect(),
        });
    }

    Ok(notes)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, save_to_file, get_notes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
