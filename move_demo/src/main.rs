
use std::thread;


fn main() {

    let v = vec![1,2,30];

    thread::spawn( move||{
                for &item in v.iter(){//&item 和item都是ok的，一般来说 越是 取引用，越可以访问其目标对象，只要给每个取引用的层次 提供了deref 方法，这里显然 Vec的元素都实现了 deref trait
                    println!("item:{}",item);
                }
    }).join();

//    drop(v);
    println!("Hello, world!");
}
