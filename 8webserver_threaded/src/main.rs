use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use threadpool::{Builder};
use std::thread;
use std::time::Duration;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let n_workers = 2;
    let pool = Builder::new()
        .num_threads(n_workers)
        .thread_stack_size(8_000_000)
        .build();
    let an_atomic = Arc::new(AtomicUsize::new(0));

    for stream in listener.incoming() {
        let an_atomic = an_atomic.clone();

        let stream = stream.unwrap();
        println!("{act} {max} {queued}",
                 act=pool.active_count(),
                 max=pool.max_count(),
                 queued=pool.queued_count());

        pool.execute(move|| {
            an_atomic.fetch_add(1, Ordering::Relaxed);

            handle_connection(stream);
            an_atomic.fetch_sub(1, Ordering::Relaxed);
            println!("Handled {an_atomic:#?}");
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}