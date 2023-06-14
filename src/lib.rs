


pub mod cmd {
    pub fn proccess(cmd: &str) {
        let cmd = cmd.to_lowercase();
        let cmd = cmd.as_str();
        match cmd {
            "help" => println!("!               delete          literal         prompt          send
?               debug           ls              put             status
append          dir             mdelete         pwd             trace
ascii           disconnect      mdir            quit            type
bell            get             mget            quote           user
binary          glob            mkdir           recv            verbose
bye             hash            mls             remotehelp
cd              help            mput            rename
close           lcd             open            rmdir"),
            "cls" | "clear" => print!("\x1B[2J\x1B[1;1H"),
            _ => println!("Please enter a valid command.\r\nIf you need a list of all the commands, type {:?}", "help"),
        }
    }
}