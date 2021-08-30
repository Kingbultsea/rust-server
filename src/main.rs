use String;

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
        email: String::from("123"),
        username: String::from("name"),
        ..new_user
    };

    println!("{} {} {} {}", new_user.email, new_user.username, new_user.active, new_user.sign_in_count);

    println!("{} {} {} {}", user3.email, user3.username, user3.active, user3.sign_in_count);

    println!("{} {} {} {}", user.email, user.username, user.active, user.sign_in_count);
}

