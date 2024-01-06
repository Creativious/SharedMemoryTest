use shared_memory::shared_memory::SharedMemory;

fn main() {
    let mut shm = SharedMemory::create("test", 1000).unwrap();
    shm.write_string("Hello World!");

    let shm_other = SharedMemory::open("test", 1000).unwrap();
    // convert the data to a string and then print it
    println!("{}", shm_other.read_string());

}
