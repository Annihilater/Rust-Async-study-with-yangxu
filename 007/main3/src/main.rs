use std::marker::PhantomPinned;
use std::pin::Pin;

// 这个例子是把它 Pin 到了堆内存
#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    // 这个标记就会让我们的类型变成 `!Unpin` 的
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = Test {
            a: String::from(txt),
            b: std::ptr::null(),
            // This makes our type `!Unpin`
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.as_ref().a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

        boxed
    }

    // fn init(self: Pin<&mut Self>) {
    //     let self_ptr: *const String = &self.a;
    //     let this = unsafe { self.get_unchecked_mut() };
    //     this.b = self_ptr;
    // }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        // 首先将b解引用，再返回它的引用
        unsafe { &*(self.b) }
    }
}

fn main() {
    let test1 = Test::new("test1");
    let test2 = Test::new("test2");

    println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
    println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
}
