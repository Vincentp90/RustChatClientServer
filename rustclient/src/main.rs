use std::net::{TcpStream};
use std::io::{self, Write, BufWriter, BufReader, BufRead};
use std::thread;

fn main() {    
    match TcpStream::connect("127.0.0.1:8888") {
        Ok(stream) => {
            println!("Successfully connected to server in port 8888");
            let rstream = stream.try_clone().expect("clone failed...");
            let mut reader = BufReader::new(rstream);
            let mut writer = BufWriter::new(stream);      
            
            thread::spawn(move|| {
                let mut line = String::new();
                loop {
                    match reader.read_line(&mut line){
                        Ok(_) => {
                            print!("{}", line);
                            line = String::new();
                        },
                        Err(_) => {
                            println!("Failed to receive data");
                            break;
                        }
                    }
                }
            });
                   
            loop{
                let mut input = String::new();
                let res = io::stdin().read_line(&mut input);
                match res {
                    Ok(_) => {
                        input = format!("{}\n",input);
                        //print!("Sending: {}", input);
                        writer.write(input.as_bytes()).unwrap();
                        writer.flush().unwrap();
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
