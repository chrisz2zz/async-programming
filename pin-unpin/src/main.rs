#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let a_ptr: *const String = &self.a;
        self.b = a_ptr;
    }

    fn a(&self) -> &str {
        self.a.as_str()
    }

    fn b(&self) -> &String {
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe {&*(self.b)}
    }
}

fn main() {
//     let mut test1 = Test::new("test1");
//     test1.init();
//     let mut test2 = Test::new("test2");
//     test2.init();

//     println!("a: {}, b: {}", test1.a(), test1.b());
//     println!("a: {}, b: {}", test2.a(), test2.b());


    // let mut test1 = Test::new("test1");
    // test1.init();
    // let mut test2 = Test::new("test2");
    // test2.init();

    // println!("a: {}, b: {}", test1.a(), test1.b());
    // std::mem::swap(&mut test1, &mut test2);
    // println!("a: {}, b: {}", test2.a(), test2.b());

    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("a: {}, b: {}", test1.a(), test1.b());
    std::mem::swap(&mut test1, &mut test2);
    test1.a = "I`ve totally changed now!".to_string();
    println!("a: {}, b: {}", test2.a(), test2.b());
}
