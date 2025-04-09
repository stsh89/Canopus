use eyre::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    task::JoinHandle,
};

#[tokio::main]
async fn main() -> Result<()> {
    let app = ShellApp::new().await?;
    let handle = start_tcp_stream(app).await;

    handle.await??;

    Ok(())
}

async fn start_tcp_stream(mut app: ShellApp) -> JoinHandle<Result<()>> {
    tokio::spawn(async move {
        loop {
            let (mut stream, addr) = app.tcp_listener.accept().await?;
            println!("Incoming connection from: {}", addr);
            app.sessions_count += 1;
            println!("Sessions count: {}", app.sessions_count);

            stream.write_all("Hello, world!".as_bytes()).await?;
            start_tcp_repl(stream).await;
            // tokio::spawn(async move {
            //     let handle = .await;
            //     handle.await??;

            //     Ok(())
            // });

            // Ok(())
        }
    })
}

async fn start_tcp_repl(mut stream: TcpStream) -> JoinHandle<Result<()>> {
    tokio::spawn(async move {
        loop {
            let mut buf = vec![0; 1024];
            match stream.read(&mut buf).await {
                Ok(n) if n == 0 => {
                    println!("Connection closed.");
                    break;
                }
                Ok(n) => {
                    // Convert the received bytes into a string
                    if let Ok(response) = std::str::from_utf8(&buf[..n]) {
                        if response == "exit" {
                            stream.write_all(b"Bye, bye").await?;

                            break;
                        } else {
                            stream.write_all(response.as_bytes()).await?;
                        }

                        println!("Received: {}", response);
                    } else {
                        eprintln!("Failed to parse server response as UTF-8.");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from server: {}", e);
                    break;
                }
            }
        }

        Ok::<(), eyre::Error>(())
    })
}

struct ShellApp {
    sessions_count: u8,
    tcp_listener: TcpListener,
}

impl ShellApp {
    async fn new() -> Result<Self> {
        Ok(ShellApp {
            sessions_count: 0,
            tcp_listener: TcpListener::bind("127.0.0.1:8080").await?,
        })
    }
}
