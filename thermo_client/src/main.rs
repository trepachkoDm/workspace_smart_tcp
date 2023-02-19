use std::{error::Error, sync::Arc, time::Duration};
use thermo_client::Temperature;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:34253";
    let socket = UdpSocket::bind(addr)
        .await
        .map_err(|e| format!("Error binding socket: {}", e))?;
    let temperature = Arc::new(Temperature::default());
    let temperature_clone = temperature.clone();

    let (sender, mut receiver) = mpsc::channel::<(usize, [u8; 10])>(100);
    let socket_task = tokio::spawn(async move {
        loop {
            let mut buf = [0; 10];
            let (amt, _src) = match socket.recv_from(&mut buf).await {
                Ok((amt, src)) => (amt, src),
                Err(e) => {
                    println!("Error receiving data: {}", e);
                    continue;
                }
            };

            let data = (amt, buf);
            if sender.send(data).await.is_err() {
                println!("Receiver dropped");
                return;
            }
        }
    });

    let receiver_task = tokio::spawn(async move {
        while let Some((amt, buf)) = receiver.recv().await {
            let char_symbol = buf[0];

            match char::from(char_symbol) {
                't' => {
                    let arr = [buf[1], buf[2], buf[3], buf[4]];
                    let val = f32::from_be_bytes(arr);
                    temperature_clone.set(val);
                }

                _ => {
                    println!(
                        "incorrect data received from {}. Expected 't' but got {}",
                        &amt,
                        char::from(char_symbol)
                    );
                }
            }
        }
    });

    for _ in 0..30 {
        println!("Client: {:?}°", temperature.get().round());
        time::sleep(Duration::from_secs(1)).await;
    }

    socket_task.abort();
    receiver_task.abort();
    Ok(())
}
