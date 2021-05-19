use std::str::FromStr;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use std::thread::spawn;
use std::{thread, time};
use std::io::{BufWriter, BufReader, BufRead};
use std::sync::{Arc,RwLock};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

fn handle_connection(stream: TcpStream, chan: Sender<String>, arc: Arc<RwLock<Vec<String>>>) {
    let rstream = stream.try_clone().expect("clone failed...");
    let mut reader = BufReader::new(rstream);
    let mut writer = BufWriter::new(stream);    
    writer.write(b"Welcome to Simple Chat Server!\n").unwrap();
    writer.write(b"Plz input yourname: \n").unwrap();
    writer.flush().unwrap();
    let mut name = String::new();
    reader.read_line(&mut name).unwrap();    
    writer.write_fmt(format_args!("Hello, {}!\n", name)).unwrap();
    writer.flush().unwrap();
    
    spawn(move|| {
        let name = name.trim_end();
        loop{
            let mut reads = String::new();
            reader.read_line(&mut reads).unwrap();
            if reads.trim().len() != 0 {
                //println!("DEBUG: reads len =>>>>> {}", reads.len());
                chan.send(format!("[{}] said: {}", name, reads)).unwrap();
                //println!("DEBUG: got '{}' from {}", reads.trim(), name);
            }
        }
    });

    let mut pos = 0;
    let sleepdur = time::Duration::from_millis(100);
    loop {
        {
            let lines = arc.read().unwrap();
            //println!("DEBUG arc.read() => {:?}", lines);
            let mut iswritten = false;
            for i in pos..lines.len() {
                writer.write_fmt(format_args!("{}", lines[i])).unwrap();
                pos = lines.len();
                iswritten = true;                               
            };
            if iswritten {
                writer.write(b" > ").unwrap();
                writer.flush().unwrap();
            }            
        }
        thread::sleep(sleepdur);
    }    
}

fn main() {
    let addr: SocketAddr = SocketAddr::from_str("127.0.0.1:8888").unwrap();
    let listener = TcpListener::bind(addr).unwrap();

    let (send, recv): (Sender<String>, Receiver<String>) = mpsc::channel();
    let arc: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(Vec::new()));

    let arc_w = arc.clone();

    spawn(move|| {
        loop {
            let msg = recv.recv().unwrap();
            //print!("DEBUG: msg {}", msg);
            {
                let mut arc_w = arc_w.write().unwrap(); // TODO arc_w keeps growing
                arc_w.push(msg);
            } // write lock is released at the end of this scope
        }
    });

    for stream in listener.incoming() {
        match stream {
            Err(_) => println!("listen error"),
            Ok(stream) => {
                println!("connection from {} to {}",
                         stream.peer_addr().unwrap(),
                         stream.local_addr().unwrap());
                let send = send.clone();
                let arc = arc.clone();
                spawn(move|| {
                    handle_connection(stream, send, arc);
                });
            }
        }
    }
}