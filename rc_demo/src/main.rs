use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List{
    container(i32,RefCell<Rc<List>>),//数值类型
    Nil        //数值类型 系统Nil 的意思
}

use List::container;
use List::Nil;

impl  List{
    fn tail (&self)->Option<&RefCell<Rc<List>>>
    {
        match self {
            container(_,item)=>{
                    Some(item)
            },
            Nil=>{
                None
            }
        }
    }

}



fn main(){
    let a = Rc::new(container(5,RefCell::new(Rc::new(Nil))));

    println!("1. a rc count={}",Rc::strong_count(&a));

    println!("result:{:#?}",a);//Display Trait 处于 Deref的哪一层，显然container实现了Display trait

}