use std::time::Duration;

use chrono::Local;
use tokio::time;

// https://learnku.com/docs/async-book/2018/async_await_primer/4788
async fn learn_and_sing() {
    // 在唱歌之前等待学歌完成
    // 这里我们使用 `.await` 而不是 `block_on` 来防止阻塞线程，这样就可以同时执行 `dance` 了。
    let song = learn_song().await;
    sing_song(song).await;
}

async fn learn_song() -> String {
    // time::sleep(Duration::from_secs(5)).await;
    let mut i1 = time::interval(Duration::from_secs(3));
    loop {
        i1.tick().await;
        print_time();
        println!("learning song");
        // String::from("ABC")
    }
}
async fn sing_song(song: String) {
    println!("singing song: {}", song);
}
async fn dance() {
    let mut i2 = time::interval(Duration::from_secs(10));
    loop {
        i2.tick().await;
        print_time();

        println!("dancing");
        // String::from("ABC")
    }
    // println!("dancing");

}
async fn async_main() {
    loop {
        let f1 = learn_and_sing();
        let f2 = dance();
        // `join!` 类似于 `.await` ，但是可以等待多个 future 并发完成
        // 如果学歌的时候有了短暂的阻塞，跳舞将会接管当前的线程，如果跳舞变成了阻塞
        // 学歌将会返回来接管线程。如果两个futures都是阻塞的，
        // 这个‘async_main'函数就会变成阻塞状态，并生成一个执行器
        futures::join!(f1, f2);
    }
}

#[tokio::main]
async fn main() {
    // block_on(async_main());
    async_main().await;
}

fn print_time() {
    let fmt = "%Y-%m-%d %H:%M:%S";
    println!("current time is : {}", Local::now().format(fmt).to_string());

}