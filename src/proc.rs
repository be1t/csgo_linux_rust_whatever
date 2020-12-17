use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Read;

pub fn find_csgo_pid() -> u32 {
    let mut pid: u32 = 0;
    for process in fs::read_dir("/proc").unwrap() {
        // Open "/proc/{process id}/comm" file
        let comm = format!("{}/comm", process.unwrap().path().display());
        let comm_path = Path::new(&comm);
        let file = File::open(&comm_path);
        match file {
            // Errors are expected for some files because not everything in
            // /proc is a process, but I don't want to panic if this happens,
            // hence the Err(_) => () (basically a no-op).
            Err(_) => (),
            Ok(mut f) => {
                let mut s = String::new();
                f.read_to_string(&mut s).unwrap();
                if s.trim() == "csgo_linux64" {
                    let split: Vec<&str> = comm.split("/").collect();
                    pid = split[2].parse().unwrap();
                    break;
                }
            }
        }
    }
    if pid == 0 {
        panic!("Couldn't find csgo... is it running?");
    }
    return pid;
}
