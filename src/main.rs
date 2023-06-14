use std::{thread::{self, sleep}, io::{self, Write}};
use std::sync::mpsc;
use std::time::Duration;
use spinoff::{Spinner, spinners, Color};

fn main() {

    // * Clear the terminal
    print!("\x1B[2J\x1B[1;1H");

    let mut buffer = String::new();
    println!("Input the IP or the DNS name of FTP server you are trying to connect to:");
    io::stdin()
        .read_line(&mut buffer)
        .expect("Read failed");
    let spinner = Spinner::new(spinners::FistBump, "Connecting to the FTP server...", Color::TrueColor { r:74, g: 246, b: 38 });
    let (tx, rx) = mpsc::channel();
    let (txr, rxr) = mpsc::channel();
    let mut buffer_cl = buffer.clone();
    thread::spawn(move || {
        let _receive = rx.recv().unwrap();
        print!("\x1B[2J\x1B[1;1H");
        buffer_cl.pop();
        buffer_cl.pop();
        let msg = &format!("Connected to the {} server", buffer_cl);
        spinner.success(msg);
        txr.send("").unwrap();
    });
    sleep(Duration::from_secs(1));
    tx.send("").unwrap();
    rxr.recv().unwrap();
    loop {
        let mut cmd = String::new();
        print!("ftp> ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Read failed");
        cmd.pop();
        cmd.pop();
        ftp_k::cmd::proccess(&cmd);
        if cmd != "cls" && cmd != "clear" {println!();}
    }
}
