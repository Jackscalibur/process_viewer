#[derive(Clone)]
struct FdEntry {
    pid: u32,
    process_name: String,
    fd: String,
    proto: String,
    remote: String,
}
