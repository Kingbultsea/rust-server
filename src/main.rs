use String;
use rand::Rng;

fn main() {
    another_function(5);
    use_if_in_let();
    println!("use_loop value {}", use_loop());
    println!("use_while value {}", use_while());
    use_while_array();
    use_for_array();
    use_for_trulp();
    use_string();
    use_stark();
    use_tuples();
    use_fn();
    use_borrow();
    use_borrow_multiple();
    use_slice_type_first_word(&String::from("hello"));
    use_slice();
    use_slice_with_array();
    use_struct();
    tuple_struct();
    example_struct();
    use_enum();
    use_enum_options();
    use_enum_option();
    use_rand_package();
    use_vector();
    use_string_utf8();
}

fn another_function(x: i32) {
    let mut _x = five();
    _x = plus_1(_x);
    _x = if_state(_x + 222);
    let _y = {
        let _y = 6;
        _x + 1
    };
    println!("i am another function {} {} {}", x, _y, _x);
}

fn five() -> i32 {
    5
}

fn plus_1(x: i32) -> i32 {
    x + 1
}

fn if_state(x: i32) -> i32 {
    if x != 22 {
        5
    } else {
        6
    }
}

fn use_if_in_let() {
    let condition = true;
    let c =
        if condition {
            5
        } else {
            6
        };
    println!("i wanna to see {}", c)
}

fn use_loop() -> i32 {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 10 {
            break 100
        }
    }
}

fn use_while() -> i32 {
    let mut count = 0;
    while count < 10 {
        count += 1;
    };
    count
}

// 一种引起恐慌的遍历方法 所以诞生了for
fn use_while_array() {
    let a = [1,2,3,4,5,6];
    let mut index = 0;
    while index < a.len() {
        println!("use_while_array {}", a[index]);
        index += 1;
    }
}

fn use_for_array() {
    let a: [i32; 6] = [1,2,3,4,5,6];
    for i in a.iter() {
        println!("use_for_array {}", i);
    }
}

fn use_for_trulp() {
    for i in (1..4).rev() {
        println!("use_for_trulp {}", i);
    }
}

fn use_string() {
    let mut s = String::from("\r\nhello");
    s.push_str(",world");
    let s2 = s.clone();
    println!("use_string {} {}", s2, s);
}

fn use_stark() {
    let a1 = 1;
    let a2 = a1.clone();
    let s = String::from("hello world");
    {
        // 不会存在什么借用的概念 就fn会
        println!("in scoped {}", s);
        let _a3 = a1;
    }
    println!("use_stark {} {} {}", a1, a2, s);
}

fn use_tuples() {
    let a = (1, 2, 3);
    let a2 = a;
    println!("use_tuples {} {}", a.0, a2.0);
}

fn use_fn() {
    let s = String::from("hello");
    use_fn_takes_ownership(&s);
    let x = "i32";
    use_fn_makes_copy(x);
    println!("use_fn {} {}", s, x);
}

fn use_fn_takes_ownership(s: &String)  {
    println!("use_fn_takes_ownership {}", s);
}

fn use_fn_makes_copy(x: &str) {
    println!("use_fn_makes_copy {}", x)
}

fn use_borrow() {
    let s1 = String::from("hello");
    // use_borrow_string(s1); 方法会拿走ownership
    let mut s = s1;
    let len = use_borrow_calculate_len(&s);
    println!("use_borrow {}", len);
    use_borrow_with_modify(&mut s);
    use_borrow_with_modify(&mut s);
    use_borrow_dangling();
    println!("use_borrow_with_modify {}", s);
}

fn use_borrow_calculate_len(x: &String) -> usize {
    x.len()
}

fn use_borrow_with_modify(x: &mut String) {
    {
        let _r1 = &x;
    }
    x.push_str(", push use_borrow_with_modify");
}

fn use_borrow_multiple() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
}

fn use_borrow_dangling() -> String {
    let s = String::from("use_borrow_dangling");
    s
}

// fn use_borrow_string(x: String) {
//
// }

fn use_slice_type_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn use_slice_second_word(s: &str) -> &str {
    let s2 = s.as_bytes();
    let mut count = 0;
    for (i, &item) in s2.iter().enumerate() {
        if item == b' ' {
            count += 1;
            if count > 1 {
                return &s[..i];
            }
        }
    }
    &s[..]
}

fn use_slice() {
    let mut s = String::from("hello world");
    let slice = &s[..s.len()];
    let slice2 = &s[..];
    let word = use_slice_second_word("heeeee llo llo");
    use_fn_makes_copy(word);
    println!("{} {} {}", slice, slice2, word);
    s.clear();
}

fn use_slice_with_array() {
    let a = [1,2,34,5,6];
    let world = &a[1..2];
    println!("{}", world[0]);
}

fn use_struct() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool
    }

    let user = User {
        email: String::from("123456@qq.com"),
        username: String::from("username"),
        active: false,
        sign_in_count: 0
    };

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: false,
            sign_in_count: 0
        }
    }

    let new_user = build_user(String::from("123@qq.com"), String::from("username"));

    let user3 = User {
        email: String::from("123 user3"),
        username: String::from("name user3"),
        ..new_user
    };

    println!("{} {} {} {}", new_user.email, new_user.username, new_user.active, new_user.sign_in_count);

    println!("{} {} {} {}", user3.email, user3.username, user3.active, user3.sign_in_count);

    println!("{} {} {} {}", user.email, user.username, user.active, user.sign_in_count);
}

fn tuple_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    println!("tuple_struct {} {}", _black.0, _origin.1);
}

# [derive(Debug)]
struct ExampleReact {
    width: i32,
    height: i32
}

impl ExampleReact {
    fn area(&mut self, second: i32) -> i32 {
        println!("area ExampleReact {}", second);
        self.width * self.height
    }

    fn can_hold(&self, another: &ExampleReact) -> bool {
        self.width > another.width && self.height > another.height
    }

    fn hello(x: &ExampleReact) {
        println!("hello {:?}", x);
    }

    fn make_react(x: i32, y: i32) -> ExampleReact {
        ExampleReact {
            width: x,
            height: y
        }
    }
}

fn example_struct() {
    let mut react1 = ExampleReact {
        width: 2_000,
        height: 4_000
    };

    let react2 = ExampleReact {
        width: 1,
        height: 1,
    };

    println!("example_struct {}", react1.area(32));
    println!("example_struct 2 {}", react1.width);
    println!("example_struct 3 {:?}", react1);
    println!("example_struct 4 {:#?}", react1);

    println!("is react2 can hold react1? {}", react2.can_hold(&react1));
    ExampleReact::hello(&react1);

    println!("is react2 can hold react1? {}", ExampleReact::can_hold(&react2, &react1));

    let react3 = ExampleReact::make_react(100, 100);
    println!("react 3 {:?}", react3);
}

fn use_enum() {
    # [derive(Debug)]
    enum IpAddKind {
        V4,
        V6
    }

    struct IpAddr {
        kind: IpAddKind,
        address: String
    }

    let ip_v4 = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1")
    };

    let ip_v6 = IpAddr {
        kind: IpAddKind::V6,
        address: String::from("::1")
    };

    # [derive(Debug)]
    enum IpAddKinkWithType {
        V4(u8, u8, u8, u8),
        V6(String)
    }

    let four = IpAddKinkWithType::V4(127, 0, 0, 1);
    let six = IpAddKinkWithType::V6(String::from("::1"));

    println!("use_enum {:?} {:?} {:?} {:?} {} {}", four, six, ip_v4.kind, ip_v6.kind, ip_v4.address, ip_v4.address);

    # [derive(Debug)]
    enum Message {
        // Quit,
        // Move { x: i32, y: i32 }, // Move includes an anonymous struct inside it. 匿名struct
        Write(String),
        // ChangeColor(i32, i32, i32)
    }

    impl Message {
        fn call(&self) {
            println!("Message from {:?}", self);
        }
    }

    let m = Message::Write(String::from("hello")); // 实例了

    m.call();
    m.call();
}

fn use_enum_options() {
    // let some_number = Some(5);
    // let some_string = Some("a string");
    // let absent_number: Option<i32> = None;

    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime(UsState),
        Quarter(String),
    }

    fn value_in_cents(coin: Coin) -> i8 {
        match coin { // match里面的内容成为arms
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime(state) => {
                println!("{:?}", state);
                5
            },
            Coin::Quarter(state) => {
                println!("{:?}", state);
                5
            }
        }
    }

    let quarter = Coin::Quarter(String::from("123"));

    value_in_cents(quarter);
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime(UsState::Alabama));
    value_in_cents(Coin::Dime(UsState::Alaska));
}

fn use_enum_option() {
    fn plus_one(x: Option<i32>) -> i32 {
        match x {
            Some(i) => i,
            _ => 2,
        }
    }

    let value = match 0u8 {
        _ => 2333
    };

    let some_v = Some(1);
    let v = if let Some(2) = some_v {
        1
    } else {
        2123_112
    };

    println!("{:?}", plus_one(Some(1)));
    println!("{:?}", plus_one(None));
    println!("{:?}", value);
    println!("{:?}", v);
}

fn use_rand_package() {
    let secret_number = rand::thread_rng().gen_range(1..=100); // 不懂 还可以 1..100
    println!("The secret number is: {}", secret_number);
}

// Vec在内存中是连接起来的
fn use_vector() {
    let v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];
    let mut v2 = Vec::new(); // 类型不用写 后续会自动判断的
    v2.push(1);
    println!("{:?} {:?} {:?}", v, v1, v2);

    let third = &v1[2];

    if let Some(third) = v1.get(2) {
        println!("我们相等了 {}", third);
    } else {
        println!("我们并不相等 {}", third);
    }

    if let Some(3) = v1.get(2) { // 返回 Option<&T>
        println!("我们相等了 {}", third);
    } else {
        println!("我们并不相等 {}", third);
    }

    // let does_not_exist = &v[100]; 程序会崩溃
    let does_not_exist = v1.get(0); // 如果有则返回Some(&element) 否则 None
    println!("{:?}", does_not_exist);

    let c = match does_not_exist {
        Some(i) => *i, // dereference operator 取消引用
        _ => 2
    };

    println!("{}", c);

    let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    v.push(1); // 不允许 如果Push 有一种情况 如果这块地方内存不够了 需要迁移到新的地方去，所以不允许你们这样搞
    println!("The first element is: {:?}", v);

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 10; //  dereference operator 取消引用
        println!("{}", i);
    }
    println!("{:?}", v); // 跟随变化 获得的还是其地址

    // 如果我们想要不同的类型，我们可以套一层enum。
    // 这些类型都是确定的
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![SpreadsheetCell::Int(3), SpreadsheetCell::Float(10.12), SpreadsheetCell::Text(String::from("String text"))];
    for i in &row {
        println!("{:?}", i);
        match i {
            SpreadsheetCell::Int(i2) => {
                println!("{}", i2);
            },
            _ => {
                println!("什么都没"); // 目前没方法把枚举里面的值取出来
            }
        };

        // 辣鸡插件 爆红了
        if let SpreadsheetCell::Float(value) = i {
            println!("取值 {}", value);
        }
    }
}

fn use_string_utf8() {
    
}
