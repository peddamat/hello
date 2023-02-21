#[link(name = "hello", kind = "raw-dylib")]
extern {
    fn add(left: usize, right: usize) -> usize ;
}

fn main() {
    unsafe {
        add(2,2);
    }
}