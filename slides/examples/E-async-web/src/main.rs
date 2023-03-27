use std::{
    any,
    future::{Future, Ready},
    io::Write,
    time::{Duration, Instant},
};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

struct Config {
    urls: Vec<String>,
}

struct Db;

struct Data;

impl Data {
    fn report(&self) {}
}

async fn load_config() -> anyhow::Result<Config> {
    unimplemented!()
}

async fn scrape<S: AsRef<str>>(urls: impl AsRef<[S]>) -> anyhow::Result<Data> {
    unimplemented!()
}

async fn run() -> anyhow::Result<()> {
    let config = load_config().await?;
    let data = scrape(&config.urls).await?;
    data.report();
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // // Set up a `tokio` runtime with default configurations
    // let runtime = tokio::runtime::Runtime::new().unwrap();
    // // Run a Future to completion
    // runtime.block_on(run());
    // runtime.shutdown_background();

    // let mut first_alarm = VerySimpleAlarm::new(Instant::now() + Duration::from_secs(3));
    // let mut snooze_alarm = VerySimpleAlarm::new(Instant::now() + Duration::from_secs(5));

    // loop {
    //     if let Poll::Ready(_) = first_alarm.poll() {
    //         println!("Beep beep beep");
    //     }
    //     if let Poll::Ready(_) = snooze_alarm.poll() {
    //         println!("You're late for work!")
    //     }
    // }

    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        tokio::task::spawn(async {
            handle_connection(socket).await?;
            Ok::<_, anyhow::Error>(())
        });
    }
}

trait VerySimpleFuture {
    type Output;
    /// Do work and check if task is completed
    /// Returns [Poll::Ready], containing the `Output` if so,
    /// [Poll::Pending] if not
    fn poll(&mut self) -> Poll<Self::Output>;
}

enum Poll<T> {
    Pending,
    Ready(T),
}

struct VerySimpleAlarm {
    alarm_time: Option<Instant>,
}

impl VerySimpleAlarm {
    fn new(alarm_time: Instant) -> Self {
        Self {
            alarm_time: Some(alarm_time),
        }
    }
}

impl VerySimpleFuture for VerySimpleAlarm {
    type Output = ();

    fn poll(&mut self) -> Poll<()> {
        match self.alarm_time {
            None => Poll::Ready(()),
            Some(alarm_time) if Instant::now() > alarm_time => {
                self.alarm_time.take();
                Poll::Ready(())
            }
            Some(_) => Poll::Pending,
        }
    }
}

// fn foo() -> impl Future<Output=u8> {
//     async {
//         5
//     }
// }

fn foo() -> impl Future<Output = u8> {
    futures::future::ready(5)
}

async fn handle_connection(socket: TcpStream) -> anyhow::Result<()> {
    let mut stream = BufReader::new(socket);
    let mut name = String::new();
    stream.read_line(&mut name).await?;
    let name = name.trim();
    stream
        .write_all(format!("Hello {name}!").as_bytes())
        .await?;
    Ok(())
}
