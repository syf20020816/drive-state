// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod lib;

use std::env::current_dir;
use std::path::Path;
use std::{
    ffi::OsString,
    fs::{read_dir, read_to_string},
};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_data(path: &str) -> String {
    read_to_string(Path::new(path)).unwrap()
}

#[tauri::command]
fn get_dir(dir_path: &str) -> Vec<String> {
    let dir_path = Path::new(dir_path);
    let dirs = read_dir(dir_path).unwrap();
    let mut res = vec![];
    for dir in dirs {
        res.push(dir.unwrap().file_name().to_str().unwrap().to_string())
    }
    res
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_data, get_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
