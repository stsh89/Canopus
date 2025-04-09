use eyre::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut stream = connect().await?;

    let mut buf = vec![0; 1024];
    stream.read(&mut buf).await?;
    let message = std::str::from_utf8(&mut buf)?;
    println!("Received: {}", message);

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        input = input.trim().to_string();
        stream.write_all(input.as_bytes()).await?;

        let mut buf = vec![0; 1024];
        stream.read(&mut buf).await?;
        let message = std::str::from_utf8(&mut buf)?;
        println!("Received: {}", message);

        if input == "exit" {
            break;
        }
    }

    Ok(())
}

async fn connect() -> Result<TcpStream> {
    loop {
        match TcpStream::connect("127.0.0.1:8080").await {
            Ok(stream) => return Ok(stream),
            Err(err) => match err.kind() {
                std::io::ErrorKind::ConnectionRefused => {
                    println!("Failed to connect to the Canopus Shell. Retrying in 5 seconds...");
                    tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
                }
                other => return Err(eyre::eyre!(other)),
            },
        }
    }
}
