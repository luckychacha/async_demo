extern crate redis;

use redis::{Client, Commands, ControlFlow, PubSubCommands};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// 创建一个应用程序 trait
trait AppState {
    fn client(&self) -> &Arc<Client>;
}

// 创建一个上下文结构，用于保存异步的 redis 客户端实例
struct Ctx {
    pub client: Arc<Client>,
}

// 实现 new 方法返回一个上下文
impl Ctx {
    fn new() -> Ctx {
        let client = Client::open("redis://localhost/").unwrap();
        Ctx {
            client: Arc::new(client),
        }
    }
}

// 为上下文实现应用程序 trait
impl AppState for Ctx {
    fn client(&self) -> &Arc<Client> {
        &self.client
    }
}

// 订阅消息，如果消息载体为 1000000001 则自动退出
// 使用多线程来处理订阅
fn subscribe(state: &impl AppState) -> thread::JoinHandle<()> {
    let client = Arc::clone(state.client());
    thread::spawn(move || {
        let mut conn = client.get_connection().unwrap();

        conn.subscribe(&["boo"], |msg| {
            let ch = msg.get_channel_name();
            let payload: String = msg.get_payload().unwrap();
            match payload.as_ref() {
                "1000000001" => ControlFlow::Break(()),
                a => {
                    println!("Channel '{}' received '{}'.", ch, a);
                    ControlFlow::Continue
                }
            }
        }).unwrap();
    })
}

// 发布订阅
// 使用多线程来处理消息发布
fn publish(state: &impl AppState) {
    let client = Arc::clone(state.client());
    thread::spawn(move || {
        let mut conn = client.get_connection().unwrap();

        for x in 0..1000000001 {
            //thread::sleep(Duration::from_millis(1));
            println!("Publish {} to boo.", x);
            let _: () = conn.publish("boo", x).unwrap();
        }
    });
}

fn main() {
    let ctx = Ctx::new();
    let handle = subscribe(&ctx);
    publish(&ctx);
    handle.join().unwrap();
}