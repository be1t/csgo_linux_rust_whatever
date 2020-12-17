use std::fs::File;
use std::path::Path;
use std::io::Read;

pub struct MemRegion {
    // There are more fields than just these three in each line of the maps
    // file, but these are the only three I care about.
    pub start: u64,
    pub perms: String,
    pub file: String
}

pub fn parse_maps_file(pid: u32) -> Vec<MemRegion> {
    // Open and read file
    let maps_file = format!("/proc/{}/maps", pid);
    let maps_file_path = Path::new(&maps_file);
    let mut maps = File::open(&maps_file_path)
        .expect("Couldn't open maps file");
    let mut maps_contents = String::new();
    maps.read_to_string(&mut maps_contents)
        .expect("Couldn't read maps file");
    let mut maps_lines: Vec<&str> = maps_contents.split("\n").collect();
    maps_lines.pop(); // Last line is empty so get rid of it

    // Parse file
    let mut mem_regions: Vec<MemRegion> = Vec::new();
    for line in &maps_lines {
        // Typical line looks something like this:
        // 7fff157e2000-7fff157e4000 r-xp 00000000 00:00 0 /path/to/file
        // ^----------^              ^--^                  ^-----------^
        // And the underlined fields are what I care about.
        let line_split: Vec<&str> = line.splitn(6, " ").collect();
        let addr_split: Vec<&str> = line_split[0].split("-").collect();
        let addr_start = u64::from_str_radix(addr_split[0], 16)
            .expect("Couldn't parse address for addr_start");
        // "file" may be blank for anonymously mapped regions, so use
        // line_split length to check
        let mut mapped_file = "--none--";
        if line_split.len() > 5 {
            let file_split: Vec<&str> = line_split[5].split("/").collect();
            mapped_file = file_split[file_split.len() - 1];
        }
        mem_regions.push(MemRegion{
            start: addr_start,
            perms: line_split[1].to_string(),
            file: mapped_file.to_string()
        });
    }

    return mem_regions;
}

// Searches for the given region (by checking perms and file) in the vector of
// MemRegions. Offset makes it easier to obtain anonymous regions that occur
// after known regions (i.e. regions that have names).
pub fn find_mem_region(mem_regions: &[MemRegion], perms: &str, file: &str,
                       offset: usize) -> MemRegion {
    let mut i = 0;
    let mut found = false;
    while i < mem_regions.len() {
        if mem_regions[i].perms == perms && mem_regions[i].file == file {
            found = true;
            break;
        }
        i = i + 1;
    }
    if found == false {
        panic!("Couldn't find given memory region!");
    }
    let result_region: &MemRegion = &mem_regions[i + offset];
    // Returning a new MemRegion instead of a pointer because I was
    // getting lifetime errors that I wasn't sure how to solve. It's small
    // enough that copy-by-value rather than copy-by-reference (probably?)
    // doesn't matter much though.
    return MemRegion{
        start: result_region.start,
        perms: result_region.perms.to_string(),
        file: result_region.file.to_string()
    };
}
