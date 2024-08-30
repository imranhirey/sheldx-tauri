use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use tauri::State;

// Function to handle incoming HTTP requests
fn handle_client(mut stream: TcpStream) {
    stream.write(b"HTTP/1.1 200 OK\r\n\r\nHello, world!").unwrap();
    stream.flush().unwrap();
}

// Start the HTTP server
#[tauri::command]
fn start_http_server(stop_flag: State<'_, Arc<AtomicBool>>) -> String {
    let stop_flag = stop_flag.inner().clone();
    stop_flag.store(false, Ordering::Relaxed); // Reset the stop flag
    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        for stream in listener.incoming() {
            if stop_flag.load(Ordering::Relaxed) {
                break;
            }
            let stream = stream.unwrap();
            handle_client(stream);
        }
    });

    "Server started".to_string()
}

// Stop the HTTP server
#[tauri::command]
fn stop_http_server(stop_flag: State<'_, Arc<AtomicBool>>) -> String {
    stop_flag.store(true, Ordering::Relaxed);
    "Server stopping".to_string()
}

// Greet function
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let stop_flag = Arc::new(AtomicBool::new(false));

    tauri::Builder::default()
        .manage(stop_flag.clone())
        .invoke_handler(tauri::generate_handler![greet, start_http_server, stop_http_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}