static mut MY_INT: u64 = 42;

fn main() {
    unsafe {
        MY_INT += 1;
        println!("{}", MY_INT);
    }
}
