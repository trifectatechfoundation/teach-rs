use std::{
    any,
    future::{Future, Ready},
    io::Write,
    sync::Arc,
    time::{Duration, Instant},
};

use futures::lock::Mutex;
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

    let mut first_alarm = VerySimpleAlarm {
        alarm_time: Instant::now() + Duration::from_secs(3),
    };
    let mut snooze_alarm = VerySimpleAlarm {
        alarm_time: Instant::now() + Duration::from_secs(5),
    };

    loop {
        if let Poll::Ready(_) = first_alarm.poll() {
            println!("Beep beep beep");
        }
        if let Poll::Ready(_) = snooze_alarm.poll() {
            println!("You're late for work!")
        }
    }

    // Bind the listener to the address
    // let listener = TcpListener::bind("127.0.0.1:6379").await?;

    // loop {
    //     // The second item contains the IP and port of the new connection.
    //     let (socket, _) = listener.accept().await.unwrap();
    //     tokio::task::spawn(async {
    //         handle_connection(socket).await?;
    //         Ok::<_, anyhow::Error>(())
    //     });
    // }
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
    alarm_time: Instant,
}

impl VerySimpleFuture for VerySimpleAlarm {
    type Output = ();

    fn poll(&mut self) -> Poll<()> {
        if Instant::now() >= self.alarm_time {
            Poll::Ready(())
        } else {
            Poll::Pending
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

use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;

type AppState = State<Arc<Mutex<Vec<String>>>>;

// #[tokio::main]
// async fn main() {
//     // set up shared state
//     let app_state = Arc::new(Mutex::new(Vec::new()));

//     // build our application with a route
//     let app = Router::new()
//         .route("/:name", get(handler))
//         .with_state(app_state);

//     // run it
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     println!("listening on {}", addr);
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

async fn handler(Path(name): Path<String>, State(past_names): AppState) -> Html<String> {
    let mut response = format!("<h1>Hello, {name}!</h1>");
    let mut past_names = past_names.lock().await;
    if !past_names.is_empty() {
        response += "<h2>Names we saw earlier:</h2>";
        past_names
            .iter()
            .for_each(|name| response += &format!("<p>{name}</p>"))
    }
    past_names.push(name);
    Html(response)
}
