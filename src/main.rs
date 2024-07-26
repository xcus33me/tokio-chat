use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader}, 
    net::TcpListener
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (mut socket, _addr) = listener.accept().await.unwrap();

    loop {
        let mut reader = BufReader::new(&mut socket);

        let mut line = String::new();
        
        let bytes_read = reader.read_line(&mut line).await.unwrap();
    
        socket.write_all(line.as_bytes()).await.unwrap();
    }
}
