extern crate chrono;

use std::{thread, time, process};
use std::io::{Read, Write};
use std::process::Command;
use chrono::prelude::*;
use std::fs::OpenOptions;

fn lock_access() {
   let mut file = std::fs::File::create("/tmp/cmus_music_monitor.lock").expect("create failed");
   file.write_all("RUNNING".as_bytes()).expect("write failed");
}

fn is_locked() -> bool {
    if std::path::Path::new("/tmp/cmus_music_monitor.lock").exists() == true {
        let mut file = std::fs::File::open("/tmp/cmus_music_monitor.lock").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        if contents == "RUNNING" {
            return true;
        }else {
            return false;
        }
    }
    return false;
}

fn sleep(timer: u64) {
    thread::sleep(time::Duration::from_secs(timer));
}

fn write_info(current_song: [String; 9]) {
    // In future the csv will be taken as a cli argument
    println!("{}", current_song.join(","));
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/home/ben/Scripts/musicMonitor/music.csv")
        .unwrap();

    if let Err(error) = writeln!(file, "{}", current_song.join(",")) {
        eprintln!("Writing to csv failed: {}", error);
    }
}

fn parse_info(info: String, mut tags: [String; 9]) -> [String; 9] {
    for i in 0..7{
        let split_by_tag: Vec<&str> = info.split(&tags[i]).collect();

        if split_by_tag.len() == 1 {
            tags[i] = "".to_string();
        }else{

            let tag_value:    Vec<&str> = split_by_tag[1].lines().collect();
            tags[i] = tag_value[0].trim().to_string();
        }
        // I decided to replace the tags string array with the songs values.
        // This is due to the tags array being the correct size.
        // This saves declaring 1 more 7 x string array, so is more effecient.
    }


    
    
    
    tags[7] = Local::now().naive_local().date().to_string();
    tags[8] = Local::now().naive_local().time().format("%H:%M").to_string();
    // Get volume needs adding here
    return tags;
}


fn get_info(mut last_title: String){
    let mut tags: [String; 9] = Default::default();
    tags[0] = "tag title".to_string();
    tags[1] = "tag artist".to_string();
    tags[2] = "tag album".to_string();
    tags[3] = "tag genre".to_string();
    tags[4] = "duration".to_string();
    tags[5] = "tag tracknumber".to_string();
    tags[6] = "tag date".to_string();

    let output = Command::new("cmus-remote")
                         .arg("-Q")
                         .output()
                         .expect("cmus-remote: cmus is not running");

    let info = String::from_utf8(output.stdout).expect("Not UTF-8");

    let info_array: Vec<&str> = info.split_whitespace().collect();

    if info_array.len() <= 0 {
        println!("Cmus is not running");
        sleep(120);
    } else if info_array[1] == "stopped" {
        println!("Cmus is stopped");
        sleep(60);
    } else if info_array[1] == "paused" {
        println!("Cmus is paused");
        sleep(60);
    } else if info_array[1] == "playing" {
        println!("Cmus is playing");
        let current_song: [String; 9] = parse_info(info, tags);
        let current: String = current_song[0].clone();
        if last_title == current {
        }else{
            last_title = current;
            write_info(current_song);
        }
        sleep(60);
    }
    get_info(last_title);
}

fn main() {
    if is_locked() == true{
        println!("All ready running!");
        process::exit(1);
    }
    lock_access();
    get_info("sdvkjsiascc982ca2c".to_string());
}
