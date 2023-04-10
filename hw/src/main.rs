
// 여기에 답안 소스만 제출해 주세요

// 1번 소스
mod hw1 {
    pub fn run() {
        let (op, lhs, rhs) = read_line().unwrap();
        let op_fn: fn(i32, i32) -> i32 = match op {
            1 => |a, b| a + b,
            2 => |a, b| a - b,
            _ => panic!(),
        };
        println!("{}", op_fn(lhs, rhs))
    }

    fn read_line() -> Option<(i32, i32, i32)> {
        use std::io::stdin;
        let mut buf = String::new();
        match stdin().read_line(&mut buf) {
            Ok(_) => {
                    let mut it = buf.trim().split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap());
                    Some((it.next().unwrap(), it.next().unwrap(), it.next().unwrap()))
                },
            _ => None
        }
    }
}


// 2번 소스
mod hw2 {
    pub fn run() {
        let v = vec![1,2,3,4,5,6,7,8,9,10];
        println!("{}", v.iter().sum::<i32>());
    }
}



// 3번 소스
mod hw3 {
    pub fn run() {
        let s1 = "A".to_string();
        let s2 = "D".to_string();
        let s3 = "X".to_string();
        let r = min(&s1, &s2, &s3);
        println!("{}", r); // "A"
    }

    fn min(s1: &String, s2: &String, s3: &String) -> char {
        *(
            [s1.chars().next().unwrap(), s2.chars().next().unwrap(), s3.chars().next().unwrap()]
            .iter().min().unwrap()
        )
    }
}


// 4번 소스
mod hw4 {
    pub fn run() {
        #[allow(unused_variables)]
        let c1 = Circle { x:3.0, y:3.0, radius:5.0 };
        let mut c2 = Circle::new(3.0, 3.0, 5.0);
        println!("{}", c2.area());
        c2.inflate(2.0);
        println!("{}", c2); // 3.0, 3.0, 7.0
    }

    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        pub fn new(x: f64, y: f64, radius: f64) -> Self {
            Circle { x, y, radius }
        }

        pub fn area(&self) -> f64 {
            self.radius.powf(2.0) * std::f64::consts::PI
        }

        pub fn inflate(&mut self, dr: f64) {
            self.radius += dr;
        }
    }

    impl std::fmt::Display for Circle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {}, {})", self.x, self.y, self.radius)
        }
    }
}


fn main() {
    hw1::run();
    hw2::run();
    hw3::run();
    hw4::run();
}

