# whisper-rs
rust code to record a voice file and then transcribe it using whisper

`whisper-rs.rs` is a Rust application that records audio for a 
specified duration and then transcribes the recorded audio using the 
Whisper API. Here's how you can use this file:

## Features

- Records audio for a customizable duration (default: 20 seconds) to recordings directory.
- Transcribes the recorded audio using Whisper tiny.
- Prints the transcribed text and saves it to transcription folder.

## Usage

To use this application, first ensure you have Rust and Cargo installed on 
your system. Then, follow these steps:

1. Clone or download this repository.

       git clone https://github.com/pnmeka/whisper-rs
3. Change into the project directory:

       cd whisper-rs
4. Ensure you have whisper

       pip install openai-whisper
6. Build and run the application using Cargo:

       cargo run

## Configuration

The duration of the recording can be changed by modifying line 20 in the 
`main.rs` file:

```rust
// Line 30: Set the recording duration (in seconds) Can be changed from default 20s
.args(["-f", "S16_LE", "-c1", "-r8000", "-d", "20", &output_filename]);
```

To use a different duration, replace `20` with your desired value. For 
example, to record for 30 seconds, change line 30 to:

```rust
.args(["-f", "S16_LE", "-c1", "-r8000", "-d", "30", &output_filename]);
```



After running these commands, the application will record audio for the 
specified duration (20 seconds by default) and then print the transcribed 
text to a text file in transcriptions.
