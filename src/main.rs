use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:1234")
        .await
        .expect("Could not bind to localhost:1234");

    loop {
        let (mut socket, _addr) = listener
            .accept()
            .await
            .expect("Could not accept socket connection");

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                line.clear();
                let bytes_read = reader
                    .read_line(&mut line)
                    .await
                    .expect("Could not read line");
                if bytes_read == 0 {
                    break;
                }

                writer
                    .write_all(line.as_bytes())
                    .await
                    .expect("Could not write line")
            }
        });
    }
}
