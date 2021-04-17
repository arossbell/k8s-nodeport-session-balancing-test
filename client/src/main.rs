// Client to the K8s single-threaded test.
// From https://github.com/arossbell/k8s-nodeport-session-balancing-test

use std::net::TcpStream;
use std::time::Instant;
use std::str::from_utf8;
use std::io::{Read, Write};

extern crate rayon;
use rayon::prelude::*;

const SRVADDR: &'static str = "10.152.183.96:5501";

fn main() {
    let time_vec = vec![2, 3, 5, 1, 10, 3, 7, 15, 3, 1, 8, 3, 5, 4];
    time_vec
        .par_iter()
        .map(
            |i|
            {
                let (ret_time, srv_name) = caller(i);
                (*i, ret_time, srv_name)
            }
        )
        .collect::<Vec<(i32, f64, String)>>()
        .iter()
        .for_each(
            |i|
            {
                println!("{} processed on {} after {} seconds.", i.0, i.2, i.1);
            }
        );
}

fn caller(
    delay: &i32,
) -> (f64, String) {
    let timer_start = Instant::now();

    let mut rcvbuffer = [0 as u8; 64];

    let recvd_hostname = match TcpStream::connect(SRVADDR) {
        Ok(mut stream) => {
            stream.write(delay.to_string().as_bytes()).unwrap();
            match stream.read(&mut rcvbuffer) {
                Ok(_) => {
                    from_utf8(&rcvbuffer).unwrap()
                },
                Err(e) => {
                    panic!("Panic while reading! {}", e)
                },
            }
        },
        Err(e) => {
            panic!("Panic while connecting! {}", e)
        },
    };

    (timer_start.elapsed().as_secs_f64(), recvd_hostname.to_string())
}
