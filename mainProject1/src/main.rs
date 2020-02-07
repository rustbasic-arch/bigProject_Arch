use std::ops::Drop;
struct Dog{
    name:String
}

impl Dog{
    fn new(name:&str)->Self{
        Dog{
            name:"".to_string()+name
        }
    }
}
impl Drop for Dog {

    fn drop(&mut self)
    {
        println!("{} die",self.name);
    }
}




//0.drop 是在离开代码块 之后执行
//1.资源释放行为逻辑方向和代码业务执行的逻辑方向 相反
fn main11() {
    let d1 = Dog::new("2号");
    let d3 = Dog::new("3号");
    {
        let d0 = Dog::new("0号");
        let d1 = Dog::new("1号");
    }//drop d2
    println!("main end");
//result;打印
    //1
    //0
    //main end
    //3
    //2

}
//drop d1


fn main() {
    let d2 = Dog::new("2号");
    let d3 = Dog::new("3号");
    {
        let d0 = Dog::new("0号");
        let d1 = Dog::new("1号");
    }//drop d2

// 提前drop

    drop(d2);
    drop(d3);

    println!("main end");
//result;打印
    //1
    //0
    //2
    //3
    //main end

}
