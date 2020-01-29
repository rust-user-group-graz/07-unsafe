unsafe fn deref() {
    let addr = 0x00;
    let ptr = addr as *const i32;
    println!("{}", *ptr);
}

fn main() {
    unsafe {
        deref();
    }
}
