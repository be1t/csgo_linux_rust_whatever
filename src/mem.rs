use std::path::Path;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;
use std::convert::TryInto;

pub fn open_mem_file(pid: u32) -> File {
    let mem_str = format!("/proc/{}/mem", pid);
    let mem_path = Path::new(&mem_str);
    let mem_file = match File::open(&mem_path) {
        Err(_) => panic!("Couldn't open memory. Are you root?"),
        Ok(file) => file,
    };
    return mem_file;
}

// Read n bytes from the given location.
// TODO: See if this is slow and if we can return pointer to vector instead?
pub fn read_bytes(mem: &mut File, location: u64, n: u64) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![0; n.try_into().unwrap()];
    mem.seek(SeekFrom::Start(location)).expect("mem.seek() fail");
    mem.read_exact(&mut buffer).expect("mem.read_exact() fail");
    return buffer;
}
