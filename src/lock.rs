use std::io::{Read, Write};

pub fn lock_access() {
    let mut file = std::fs::File::create("/tmp/cmus_music_monitor.lock").expect("create failed");
    file.write_all("RUNNING".as_bytes()).expect("write failed");
}

pub fn is_locked() -> bool {
    if std::path::Path::new("/tmp/cmus_music_monitor.lock").exists() {
        let mut file = std::fs::File::open("/tmp/cmus_music_monitor.lock").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        if contents == "RUNNING" {
            return true;
        }
    }
    false
}
