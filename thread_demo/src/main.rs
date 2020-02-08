

use std::thread;
use std::time;
use std::time::Duration;

fn main1() {

    thread::spawn(||{
        for i in 1..10{
            println!("i:{} in spawn",i);
            thread::sleep(Duration::from_millis(1));
        }

    });

    for i in 1..5{
        println!("i {} in main ",i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("Hello, world!");
}

fn main222(){

    let h = thread::spawn(||{
       for i in 0..10{
            println!("i={} in spawn ...",i);
            thread::sleep(Duration::from_millis(1));
       }
    });

    h.join();//由于join 堵塞 ：分离了 不规则的调度
    println!("=================.....======分界了..11111111111..=====================");
   thread::spawn(||{
        for i in 0..10{
            println!("i={} in spawn ...",i);
            thread::sleep(Duration::from_millis(1));
        }
    }).join();
//    join();//由于join 堵塞 ：分离了 不规则的调度


    println!("=================.....======分界了..22222222222..=====================");

    for i in 0..5{
        println!("i:{} in main",i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("hello world .....");

}

//将 join放到最后，
fn main(){
    let h = thread::spawn(||{
        for i in 1..10{
            println!("i:{} in spawn",i);
            thread::sleep(Duration::from_millis(1));
        }

    });

    for i in 1..5{
        println!("i {} in main ",i);
        thread::sleep(Duration::from_millis(1));
    }
    h.join();//放到哪里 等到哪里
    println!("Hello, world!");

}