use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use std::io::Write;
use std::io::Read;

use std::time;
use std::time::Duration;
use std::io::ErrorKind;

fn doWriteRotine(safeB:Arc<Mutex<TcpStream>>)->JoinHandle<()>{

     thread::spawn(move ||{
          let mut clientStream = safeB.lock().unwrap();

          loop{
               match clientStream.write("hellowrold".as_bytes())
                   {
                        Ok(len)=>{

                        },
                        Err(errMsg)=>{
                             match errMsg.kind() {
                                  ErrorKind::ConnectionAborted=>{
                                       println!("client:{:#?}",clientStream.peer_addr());
                                       return ;//退出
                                  }
                                  _=>{

                                  }

                             }
                        }
                   }

               thread::sleep(time::Duration::from_millis(1000));
          }
     })
}


fn doReadRotine(safeB:Arc<Mutex<TcpStream>>)->JoinHandle<()>{

     thread::spawn(||{
               println!("will read from client ....");

     })
}

fn main() {
     let ip = "127.0.0.1";
     let port = 9000;
     let addr = format!("{}:{}",ip,port);
     let listener = TcpListener::bind(addr.clone()).unwrap();
     println!("server start  at :{}",addr.clone());
     for stream in listener.incoming(){
          match stream{
               Ok(mut conn)=>{
                    let safeArc = Arc::new(Mutex::new(conn));
                    doWriteRotine(safeArc.clone());
//                    doReadRotine(safeArc.clone());
               },
               Err(s)=>{

               }

          }
     }

}
