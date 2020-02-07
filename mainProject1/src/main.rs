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




//drop 是在离开代码块 之后执行
//资源释放行为逻辑方向和代码业务执行的逻辑方向 相反
fn main() {
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