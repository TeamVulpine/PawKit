use pawkit_fs::Vfs;

fn main() {
    for directory in Vfs::working(".").unwrap().list_subdirectories().unwrap() {
        println!("{:?}", directory);
    }
}
