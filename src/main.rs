use tokio::time::{self, Instant, Duration};
use rand::prelude::*;

use ring::digest::{Context, Digest, SHA512};

mod phase;
use phase::Phaser;

use std::thread::available_parallelism;
use std::{net::SocketAddr, sync::Arc};
//use std::sync::mpsc::channel;

use socket2::{Socket, Domain, Type};
use tokio::net::UdpSocket;

/// get the local ip address, return an `Option<String>`. when it fail, return `None`.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dpa = available_parallelism().unwrap().get();
    println!("{}", dpa);
    let socket = Socket::new(Domain::IPV6, Type::DGRAM, None)?;
    let address: SocketAddr = "[2603:8000:7e00:9200:c53:ef56:1d6c:fc2e]:11001".parse().unwrap();
    socket.bind(&address.into())?;
    socket.set_nonblocking(true)?;
    let sockette = UdpSocket::from_std(socket.into())?;
    let mother = Arc::new(sockette);
    for n in 1..4 {
      let clone = mother.clone();
      tokio::spawn(async move{
        let mut buf = [0; 1024];
        let frequency:f64 = 0.5;
        let mut phaser = Phaser::new(frequency);
        let mut interval = time::interval(Duration::from_millis(100));
        let mut rng = rand::thread_rng();
        let mut bytes: i128;
        loop {
          //let (length, sender) = clone.recv_from(&mut buf).await; 
          //println!("{:?} bytes received from {:?}", length, sender);
          let tock:bool = phaser.tick(); 
          if tock {
          //  println!("TOCK") 
          } else {
            bytes = rng.gen();
            let data = bytes as u8;
            let mut hash_context = Context::new(&SHA512);
            hash_context.update(&[data]);
            hash_context.finish();

          }
        }
      });
    }

    let frequency:f64 = 0.5;
    let mut phaser = Phaser::new(frequency);
    
    // async simulator
    let mut interval = time::interval(Duration::from_millis(100));
    let mut rng = rand::thread_rng();
    let mut bytes: i128;
    loop {
    //    interval.tick().await;
     //   let (amt, src) = socket.recv_from(&mut buf)?;
        let tock:bool = phaser.tick(); 
        if tock {
            println!("Tokidoki Process Main") 
        } else {
 //           bytes = rng.gen();
 //           let data = bytes as u8;
  //          let mut hash_context = Context::new(&SHA512);
    //        hash_context.update(&[data]);
      //      hash_context.finish();

        }
    }
}

