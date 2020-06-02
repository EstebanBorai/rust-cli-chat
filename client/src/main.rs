use std::io::{ErrorKind, Read, Write, stdin};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
  let mut client = TcpStream::connect(LOCAL).expect("Failed to connect");
  client.set_nonblocking(true).expect("Failed to set not blocking");

  let (sender, receiver) = mpsc::channel::<String>();

  thread::spawn(move || loop {
    let mut buff = vec![0; MSG_SIZE];
    match client.read_exact(&mut buff) {
      Ok(_) => {
        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
        println!("Received: {:?}", msg);
      },
      Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
      Err(_) => {
        println!("Connection closed!");
        break;
      }
    }

    match receiver.try_recv() {
      Ok(msg) => {
        let mut buff = msg.clone().into_bytes();
        buff.resize(MSG_SIZE, 0);
        client.write_all(&buff).expect("Writting to socket failed");
        println!("Message sent!");
      },
      Err(TryRecvError::Empty) => (),
      Err(TryRecvError::Disconnected) => break
    }

    thread::sleep(Duration::from_millis(100));
  });

  println!("Write a message: ");

  loop {
    let mut buff = String::new();
    stdin().read_line(&mut buff).expect("Reading from stdin failed!");
    
    let msg = buff.trim().to_string();
    
    if msg == ":quit" || sender.send(msg).is_err() {
      break;
    }
  }
}
