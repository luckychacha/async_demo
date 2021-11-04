use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::net::{TcpListener, TcpStream};

use bytes::Bytes;
use mini_redis::{Connection, Frame, Result, client};
use tokio::time::sleep;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

// async fn hello() {
//     println!("hello world!");
// }

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");
    
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (tcp_stream, socket_addr) = listener.accept().await.unwrap();
        println!("socket_addr{:?}", socket_addr);
        let db = db.clone();

        tokio::spawn(async move {
            sleep(Duration::from_secs(1)).await;
            process(tcp_stream, db).await;
        });
    }
    // a.await;
    // Ok(())
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    let mut conn = Connection::new(socket);
    while let Some(frame) = conn.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("Ok".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => {
                panic!("unimplemented: {:?}", cmd)
            }
            // println!("Got {:?}", frame);

            // let response = Frame::Error("unimplemented".to_string());
            // conn.write_frame(&response).await.unwrap();
        };
        conn.write_frame(&response).await.unwrap();
    }

}