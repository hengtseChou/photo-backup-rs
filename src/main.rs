use clap::{Arg, ArgAction, Command};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command as ProcessCommand;
use std::time::{UNIX_EPOCH};
use chrono::{Datelike, NaiveDateTime};

/// Recursively calculates the total size of a directory
fn calculate_dir_size(dir: &Path) -> u64 {
    let mut total_size = 0;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                total_size += calculate_dir_size(&path);
            } else if let Ok(metadata) = fs::metadata(&path) {
                total_size += metadata.len();
            }
        }
    }

    total_size
}

/// Converts bytes to human-readable format (like `du -sh`)
fn format_size(bytes: u64) -> String {
    let sizes = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut index = 0;

    while size >= 1024.0 && index < sizes.len() - 1 {
        size /= 1024.0;
        index += 1;
    }

    format!("{:.2} {}", size, sizes[index])
}


fn main() {
    let matches = Command::new("photo-backup-rs")
        .version("0.1.0")
        .about("Syncs iPhone photos and organizes them into year-month folders")
        .arg(Arg::new("source")
            .short('s')
            .long("source")
            .required(true)
            .num_args(1)  // <-- Fix here
            .help("Specify the source directory"))
        .arg(Arg::new("destination")
            .short('d')
            .long("dest")
            .required(true)
            .num_args(1)  // <-- Fix here
            .help("Specify the destination directory"))
        .arg(Arg::new("less_output")
            .short('l')
            .long("less")
            .action(ArgAction::SetTrue)
            .help("Show less output"))
        .get_matches();


    
    let source_dir = matches.get_one::<String>("source").unwrap();
    let dest_dir = matches.get_one::<String>("destination").unwrap();
    let less_output = matches.get_flag("less_output");


    if !Path::new(source_dir).is_dir() {
        eprintln!("[ERROR] Source directory '{}' does not exist", source_dir);
        std::process::exit(1);
    }
    if !Path::new(dest_dir).is_dir() {
        eprintln!("[ERROR] Destination directory '{}' does not exist", dest_dir);
        std::process::exit(1);
    }

    let record_path = format!("{}/.processed_subfolders", dest_dir);
    let mut processed_subfolders = HashSet::new();

    if let Ok(file) = File::open(&record_path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(subfolder) = line {
                processed_subfolders.insert(subfolder);
            }
        }
    }

    println!("[INFO] Backup started");

    let folders: Vec<_> = fs::read_dir(source_dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect();

    for folder in &folders {
        let subfolder_name = folder.file_name().unwrap().to_string_lossy().to_string();
        
        if processed_subfolders.contains(&subfolder_name) {
            if !less_output {
                println!("--> Skipping fully processed subfolder: {}", subfolder_name);
            }
            continue;
        }

        let files: Vec<_> = fs::read_dir(folder)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .collect();

        for (i, file) in files.iter().enumerate() {
            let metadata = file.metadata().unwrap();
            let modified = metadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let datetime = NaiveDateTime::from_timestamp_opt(modified as i64, 0).unwrap();
            let year_month = format!("{}/{:04}-{:02}", dest_dir, datetime.year(), datetime.month());
            fs::create_dir_all(&year_month).unwrap();

            let file_path = file.path();
            if !less_output {
                print!("\r--> Processing {}: file {}/{}", subfolder_name, i + 1, files.len());
            }
            run_rsync(file_path.to_str().unwrap(), &year_month);
        }

        println!("\n--> {} completed 🚀", subfolder_name);
        processed_subfolders.insert(subfolder_name.clone());
    }

    let mut record_file = File::create(&record_path).unwrap();
    for subfolder in &processed_subfolders {
        writeln!(record_file, "{}", subfolder).unwrap();
    }

    println!("[INFO] Backup completed");
    println!("[INFO] Calculating backup storage usage...");
    let total_size = calculate_dir_size(Path::new(dest_dir));
    println!("[INFO] Backup storage usage: {}", format_size(total_size));
}
