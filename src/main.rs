extern crate clap;
use clap::{Arg, App};

use std::net::UdpSocket;
use std::thread;
use std::time::{Duration, Instant};


fn duration_to_ms(d: Duration) -> f64 {
    d.subsec_nanos() as f64 / 1000000.0
}

fn run_server () {

    let socket = UdpSocket::bind("0.0.0.0:5201").expect("couldn't bind to address");

    let mut buf = [0; 16384];
    let mut send_buff: Vec<u8> = Vec::with_capacity(16384);

    loop {
        let (number_of_bytes, src_address) = socket.recv_from(&mut buf).expect("Server: didn't receive data");

        for i in 0..number_of_bytes {
            send_buff.push(buf[i] + 1);
        }

        socket.send_to(&send_buff, &src_address).unwrap();
        send_buff.clear();
    }
}

fn run_client(ip: &str, w: u64, s: usize, n: usize) {
    let mut arr: Vec<u8> = Vec::with_capacity(s);
    let mut rsv: Vec<u8> = Vec::with_capacity(s + 1);
    let mut times: Vec<f64> = Vec::with_capacity(n);

    for _ in 0..s {
        arr.push(0);
        rsv.push(0);
    }

    let ip = String::from(ip);
    let socket = UdpSocket::bind("0.0.0.0:5202").expect("couldn't bind to address");

    for num in 0..n {
        for i in 0..s {
            arr[i] = num as u8;
        }

        let time = Instant::now();

        socket.send_to(&arr, &ip).unwrap();
        let (_, _) = socket.recv_from(&mut rsv).expect("Client: didn't receive data");

        let duration = time.elapsed();

        if rsv[0] == ((num + 1) & 0xff) as u8 {
            println!("[{:3}/{:3}]duration: {:12.6} ms", num, n, duration_to_ms(duration));
            times.push(duration_to_ms(duration));
        }
        else {
            println!("TRY IS FAILED !!!");
        }

        thread::sleep(Duration::from_millis(w));
    }

    let mut sum = 0.0;
    let mut val_max = times[0];
    let mut val_min = times[0];

    for i in 0..n {
        sum += times[i];

        val_min = if val_min > times[i] { times[i] } else { val_min };
        val_max = if val_max < times[i] { times[i] } else { val_max };
    }
    println!("REPORT:");
    println!("   min: {:12.6} ms", val_min);
    println!("   max: {:12.6} ms", val_max);
    println!("  mean: {:12.6} ms", sum / n as f64);
}

fn main() {
    let matches = App::new("Client/Server")
        .version("1.0")
        .author("Morozov Andrey")
        .about("latency timing")
        .arg(Arg::with_name("server")
            .short("s")
            .help("run server"))
        .arg(Arg::with_name("client")
            .short("c")
            .help("run client")
            .takes_value(true))
        .arg(Arg::with_name("wait")
            .short("w")
            .help("wait before send next package")
            .takes_value(true))
        .arg(Arg::with_name("size")
            .short("l")
            .help("package size")
            .takes_value(true))
        .arg(Arg::with_name("number")
            .short("n")
            .help("number of request")
            .takes_value(true))
        .get_matches();


    if matches.is_present("server") {
        println!("Running SERVER");
        run_server()
    } else if matches.is_present("client"){
        println!("Running CLIENT");

        let ip: &str = matches.value_of("client").unwrap();
        let w: u64 = matches.value_of("wait").unwrap().parse().unwrap();
        let s: usize = matches.value_of("size").unwrap().parse().unwrap();
        let n: usize = matches.value_of("number").unwrap().parse().unwrap();

        println!("            wait (ms): {:?}", w);
        println!(" package size (bytes): {:?}", s);
        println!("   number of requests: {:?}", n);

        run_client(&ip, w, s, n);
    } else {
        println!("Nothing to run !");
    }
    println!("Done");
}
