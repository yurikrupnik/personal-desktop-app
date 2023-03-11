// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn kind(name: &str) -> String {
    use std::process::Command;
    use std::path::Path;
    // let task1 = Command::new("kind").args(["create", "cluster"])
    // let task1 = Command::new("kubectl").args(["cluster-info"])
    // .spawn()
    // // .wait()
    // .expect("failed to create kind cluster");
    let path = Path::new("Taskfile.");
    if path.exists() {
        println!("File exists!");
    } else {
        println!("File does not exist!");
    }
    // let task1 = Command::new("ls").spawn().expect("failed ls");
    // let a = Command::new("pwd").spawn().expect("failed ls");
    // let task12 = Command::new("kubectx").spawn().expect("failed kubectx");
    // let task112 = Command::new("task").args(["-aj"]).spawn().expect("failed kubectx");
  // let sd = task112.stdin;
  // println!("task112.stdin, {sd:?}");
  format!("Hello, You've been greeted from Rust and leptos!")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, kind])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
