use tokio::net::{TcpListener, TcpStream};


use mini_redis::{Connection, Frame, Result, client};


// async fn hello() {
//     println!("hello world!");
// }
// https://tokio.rs/tokio/tutorial/spawning

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // let a = hello();
    // a.await;

    // // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;
    client.set("world", "hello".into()).await?;

    // // Get key "hello"
    // let result = client.get("hello").await?;

    // println!("got value from the server; result={:?}", result);

    // let result2 = client.get("world").await?;
    // println!("got value from the server; result={:?}", result2);


    // let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    // loop {
    //     let (tcpStream, socketAddr) = listener.accept().await.unwrap();
    //     process(tcpStream).await;
    // }
    // a.await;
    Ok(())
}

async fn process(socket: TcpStream) {
    let mut conn = Connection::new(socket);
    if let Some(frame) = conn.read_frame().await.unwrap() {
        println!("Got {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        conn.write_frame(&response).await.unwrap();
    }

}