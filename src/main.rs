use std::{thread, io::{self, Write}, sync::mpsc::{Sender, Receiver}};
use std::sync::mpsc;
use spinoff::{Spinner, spinners, Color};
use ftp::FtpError;

fn main() {

    // * Clear the terminal
    print!("\x1B[2J\x1B[1;1H");
    let mut connection;
    loop {
        let mut buffer = String::new();
        println!("Input the IP or the DNS name of FTP server you are trying to connect to:");
        io::stdin()
            .read_line(&mut buffer)
            .expect("Read failed");
        let spinner = Spinner::new(spinners::FistBump, "Connecting to the FTP server...", Color::TrueColor { r:74, g: 246, b: 38 });
        let (tx, rx): (Sender<Result<bool, FtpError>>, Receiver<Result<bool, FtpError>>) = mpsc::channel(); // * The result is an integer so the compiler stops bothering 
        let (txr, rxr) = mpsc::channel();
        let mut buffer_cl = buffer.clone();
        thread::spawn(move || {
            let receive = rx.recv().unwrap();
            print!("\x1B[2J\x1B[1;1H");
            buffer_cl.pop();
            buffer_cl.pop();
            match receive {
                Ok(_) => {

                    let msg = &format!("Connected to the {} server", buffer_cl);
                    spinner.success(msg);
                }
                Err(e) => {
                    let msg = &format!("An error has ocurred connecting to the {} server: {}", buffer_cl, e);
                    spinner.fail(msg);
                }
            }
            txr.send("").unwrap();
        });
        connection = ftp_k::open_connection(format!("{buffer}:21"));
        match connection {
            Ok(_) => {
                tx.send(Ok(true)).unwrap();
                rxr.recv().unwrap();
                break;
            },
            Err(err) => {
                tx.send(Err(err)).unwrap();
                rxr.recv().unwrap();
            }
        }
    }
    let connection = ftp_k::Connection::new(connection.unwrap());

    loop {
        let mut cmd = String::new();
        print!("ftp> ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Read failed");
        cmd.pop();
        cmd.pop();
        connection.proc(&cmd);
        if cmd != "cls" && cmd != "clear" {println!();}
    }
}
