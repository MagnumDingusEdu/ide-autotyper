use std::{fs, process, thread};
use std::path::PathBuf;
use std::time::Duration;
use enigo::{Enigo, self, KeyboardControllable, Key};
use native_dialog::{FileDialog, MessageDialog, MessageType};

fn show_fatal_error(message: &str) {
    MessageDialog::new()
        .set_title("Error")
        .set_text(message)
        .set_type(MessageType::Error)
        .show_alert()
        .unwrap();
}

fn show_info_message(message: &str) {
    MessageDialog::new()
        .set_title("Info")
        .set_text(message)
        .set_type(MessageType::Info)
        .show_alert()
        .unwrap();
}

fn file_to_keystrokes(filename: PathBuf) {
    let file_contents = fs::read_to_string(filename.clone());

    let file_contents = match file_contents {
        Ok(content) => content,
        _ => {
            show_fatal_error(&*format!("Failed to parse {:?} to string. Please ensure to input a text-only file.", filename));
            process::exit(1);
        }
    };

    let mut typewriter = Enigo::new();

    for line in file_contents.split("\n") {
        if line == "" {
            typewriter.key_click(Key::Return);
            typewriter.key_click(Key::Home);
        } else {
            typewriter.key_click(Key::Home);
            for ch in line.chars() {
                typewriter.key_sequence(&*ch.to_string());
                if ch == '(' || ch == '[' || ch == '{' {
                    typewriter.key_click(Key::Delete);
                }
            }
            typewriter.key_click(Key::Return);
            typewriter.key_click(Key::Home);

            thread::sleep(Duration::from_millis(100));
        }
    }

    show_info_message("The program has completed successfully.");
}


fn main() {
    let start_confirm = MessageDialog::new()
        .set_title("Auto typer")
        .set_text("Welcome to the IDE auto-typer! Press YES to choose a file to input.")
        .set_type(MessageType::Info)
        .show_confirm()
        .unwrap();
    if !start_confirm {
        return;
    }
    let path = FileDialog::new()
        .set_location("./")
        .show_open_single_file()
        .unwrap();

    let path = match path {
        Some(path) => path,
        None => return,
    };


    let yes = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Are you sure ?")
        .set_text(&format!("Press YES to begin. \nThe program will start typing after a 5-second pause to allow you to switch windows."))
        .show_confirm()
        .unwrap();

    if yes {
        thread::sleep(Duration::from_secs(5));

        file_to_keystrokes(path);
    }
}
