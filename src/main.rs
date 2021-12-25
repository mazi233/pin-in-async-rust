// struct Test {
//     a: String,
//     b: *const String,
// }
//
// impl Test {
//     fn new(txt: &str) -> Self {
//         Test {
//             a: txt.to_string(),
//             b: std::ptr::null(),
//         }
//     }
//
//     fn init(&mut self) {
//         let self_ref: *const String = &self.a;
//         self.b = self_ref;
//     }
//
//     fn b(&self) -> &String {
//         unsafe {&(*self.b)}
//     }
// }
//
// fn main() {
//     let mut test1 = Test::new("test1");
//     test1.init();
//
//     let mut test2 = Test::new("test2");
//     test2.init();
//
//     println!("a: {}, b: {}", test1.a, test1.b());
//
//     std::mem::swap(&mut test1, &mut test2);
//
//     test1.a = "modified".to_string();
//
//     println!("a: {}, b: {}", test1.a, test1.b());
//     println!("a: {}, b: {}", test2.a, test2.b());
//
//     // let mut a = String::from("s1");
//     // let _test = Test {
//     //     a,
//     //     b: &a,
//     // };
//     // let mut s2 = String::from("s2");
//     // std::mem::swap(&mut s1, &mut s2);
//     // println!("s1={}, s2={}", s1, s2);
// }

// below code running at unstable rust
// please switch to unstable rust version
#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;
fn main() {
    let mut generator = || {
        yield 1;
        return "foo"
    };

    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Yielded(1) => (),
        _ => panic!("unexpected value from resume"),
    }
    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Complete("foo") => {},
        _ => panic!("unexpected value from resume"),
    }

    let mut generator = || {
        println!("2");
        yield;
        println!("4")
    };

    println!("1");
    Pin::new(&mut generator).resume(());
    println!("3");
    Pin::new(&mut generator).resume(());
    println!("5");




    let ret = "foo";
    let mut generator = move || {
        yield 1;
        return ret;
    };
    Pin::new(&mut generator).resume(());
    Pin::new(&mut generator).resume(());

    // 下面代码为上面代码简单去糖后代码
    let ret = "foo";
    let mut generator = {
        #[derive(Debug)]
        enum __Generator {
            Start(&'static str),
            Yield1(&'static str),
            Done,
        }

        impl Generator for __Generator {
            type Yield = i32;
            type Return = &'static str;

            fn resume(mut self: Pin<&mut Self>, resume: ()) -> GeneratorState<Self::Yield, Self::Return> {
                use std::mem;
                // *self -> 先解引用拿到__Generator，因为当前self是Pin<&mut Self>，和__Generator并非同一类型s
                match mem::replace(&mut *self, __Generator::Done) {
                    __Generator::Start(s) => {
                        *self = __Generator::Yield1(s);
                        GeneratorState::Yielded(1)
                    }
                    __Generator::Yield1(s) => {
                        *self = __Generator::Done;
                        GeneratorState::Complete(s)
                    }
                    __Generator::Done => {
                        panic!("generator resumed after completion")
                    }
                }
            }
        }
        __Generator::Start(ret)
    };

    Pin::new(&mut generator).resume(());
    Pin::new(&mut generator).resume(());
}
