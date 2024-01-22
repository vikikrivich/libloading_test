use libloading::{Library, Symbol};

fn main() {
    unsafe {
        let lib = Library::new("src/lib.so").expect("Failed to load lib.so");

        let print_hello: Symbol<unsafe extern "C" fn()> = lib.get(b"print_hello\0")
            .expect("Failed to load print_hello function");

        print_hello();
    }
}
