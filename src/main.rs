use std::process::Command;
use std::io::{self, Write};
use chrono::Local;
use std::fs;
use std::path::Path;



fn main() -> io::Result<()> {

    //make sure a dir called recordings exists
    let path = Path::new("recordings");
    if !path.exists() {
        fs::create_dir(path)?;
        println!("Created a recording folder");
    }
    let path = Path::new("transcription");
    if !path.exists() {
        fs::create_dir(path)?;
        println!("Created a transcription folder");
    }

    let recording_name=get_recording_name();
    let current_date= Local::now().format("%Y%m%d").to_string();
    let output_filename=format!("recordings/{}_{}.wav", recording_name, current_date);

    println!("Recording to file: {}", output_filename);

    let output= Command::new("arecord")
        .args(["-f", "S16_LE", "-c1", "-r8000", "-d", "20", &output_filename])
        .output()
        .expect("Error occurred");
    if output.status.success() {
        println!("Successfully completed. Will start transcribing");
    } else {
        eprintln!("Recording failed: {}", String::from_utf8_lossy(&output.stderr));
        return Ok(());
    }

    let transcribe=Command::new("whisper")
        .args([&output_filename, "--model", "tiny", "--output_dir", "transcription"])
        .output()
        .expect("Error occurred");
    if transcribe.status.success() {
        println!("Successfully completed transcribing. Please correct errors");
    } else {
        eprintln!("Transcription failed: {}", String::from_utf8_lossy(&transcribe.stderr));
    }
    
    Ok(())
    
}

fn get_recording_name() -> String {
    print!("Enter a name of the recording: ");
    io::stdout().flush().unwrap();

    let mut input =String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}
