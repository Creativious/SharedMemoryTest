use shared_memory::shared_memory::SharedMemory;

fn main() {
    let mut shm = SharedMemory::create("test", 5).unwrap();
    shm.write_string("Hello World!");

    let mut shm_other = SharedMemory::open("test", 5).unwrap();
    // convert the data to a string and then print it
    println!("{}", shm_other.read_string());

}
