use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::time;
use std::thread;

fn main() {
    // server
    let handle_server = thread::spawn(move||{
        println!("[Server] : start...");
        worker_server();
        println!("[Server] : close...");
    });

    // client
    let handle_client = thread::spawn(move||{
        // delay 500ms
        thread::sleep(time::Duration::from_millis(500));
        println!("[Client] : start...");
        worker_client();
        println!("[Client] : close...");
    });

    // thread join
    handle_server.join();
    handle_client.join();
}

fn worker_server() {
    // 绑定IP、端口
    let listener = TcpListener::bind("127.0.0.1:8888");
    if listener.is_err() {
        println!("[Server]: Bind ip and port fail...");
        return;
    }

    let listener = listener.unwrap();
    println!("[Server]: Waiting for next message...");
    for stream in listener.incoming() {
        if stream.is_err() {
            println!("[Server]: Getting error message...");
            return;
        }

        let mut stream = stream.unwrap();
        // process stream
        println!("[Server]: Processing error message...");
        if process_stream(stream) {
            println!("[Server]: Success processed message...");
        }

        println!("[Server]: Waiting for next message...");
    }

}

fn process_stream(mut stream : TcpStream)  -> bool{
    let mut buffer = [0; 1024];
    if stream.read(&mut buffer).is_err() {
        return false;
    }

    println!("[Server]: Get request info : {}", String::from_utf8_lossy(&buffer[..]));
    if stream.write("Server has received your request".as_ref()).is_err() {
        return false;
    }

    return true;
}

fn worker_client() {
    // 绑定IP、端口
    let stream = TcpStream::connect("127.0.0.1:8888");
    //
    if stream.is_err() {
        println!("[Client]: Connect fail...");
        return;
    }

    let mut stream = stream.unwrap();
    let status =stream.write("client send info to seaver".as_ref());
    if status.is_err() {
        println!("[Client]: Send info fail...");
        return;
    }

    let mut buffer = [0; 1024];
    if stream.read(&mut buffer).is_err() {
        println!("[Client]: Recv info fail...");
        return;
    }

    println!("[Client]: Get message from server : {}", String::from_utf8_lossy(&buffer[..]));

    stream.shutdown(Shutdown::Both);
}
