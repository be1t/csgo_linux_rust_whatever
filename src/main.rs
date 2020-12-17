mod proc;
mod maps;
use maps::MemRegion;
mod mem;
mod offsets;

fn main() {
    // Find csgo_linux64.
    let pid = proc::find_csgo_pid();

    // Parse /proc/pid/maps file.
    // A lot of this is taken from earlier work: github.com/giggybyte/rmrw
    let mem_regions: Vec<MemRegion> = maps::parse_maps_file(pid);

    // Open /proc/pid/mem for reading and writing.
    let mut mem = mem::open_mem_file(pid);
    
    // Bunnyhop test
    let region = maps::find_mem_region(
        &mem_regions,
        "rw-p",
        "client_client.so",
        1);
    // This address is a pointer to another address, the start of the local
    // player.
    let local_player_ptr: u64 = region.start + offsets::LOCAL_PLAYER_PTR;
    let local_player_bytes = mem::read_bytes(&mut mem, local_player_ptr, 8);

    for byte in local_player_bytes {
        println!("{:02x}", byte);
    }
}
