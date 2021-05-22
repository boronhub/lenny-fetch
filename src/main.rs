mod spec;
mod utils;
use spec::Info;
use rand::seq::SliceRandom;
use std::env::{args, Args};
use std::io::Result;
use std::process;
use crate::utils::pusher;

fn main() {
    let data = Info::new();
    if let Err(e) = data.run(args()) {
        eprintln!("{}", e);
    }
}

impl Info {
    pub fn run(self, args: Args) -> Result<()> {
        let mut info: Vec<String> = vec![];
        info.push("".to_string());
        let mut count = 0;
        if let Some(v) = Self::from_env("USER") {
            let uname = v;
            count += uname.len();
            if let Some(u) = Self::get_cmd("hostname", "f") {
                let hname = u;
                count += hname.len() + 1;
                info.push(format!("{}@{}\t\t", uname, hname))
            } else {
                info.push(format!("\t\t{}\t\t", uname))
            }
        }

        info.push(format!("{}", "=".repeat(count)));
        info.push("".to_string());

        if args.len() == 1 {
            pusher(&self.distro, &mut info);
            pusher(&self.shell, &mut info);
            pusher(&self.term, &mut info);
            pusher(&self.session, &mut info);
            pusher(&self.kernel, &mut info);
            pusher(&self.cpu, &mut info);
            pusher(&self.uptime, &mut info);
            pusher(&self.gpu, &mut info);
            pusher(&self.host_device, &mut info);
            pusher(&self.mem, &mut info);
        } else {
            args.into_iter()
                .skip(1)
                .try_for_each(|arg| self.parse_args(&arg, &mut info))?;
        }

        info.push("".to_string());
        self.print_faces(&info);

        Ok(())
    }

    fn parse_args(&self, arg: &str, info: &mut Vec<String>) -> Result<()> {
        if !arg.contains('-') {
            println!("missing arguments");
        }

        if let "--help" | "-h" = arg {
            println!("{}", HELP);
            process::exit(0)
        }

        match arg {
            "--distro" | "-d" => pusher(&self.distro, info),
            "--shell" | "-s" => pusher(&self.shell, info),
            "--desktop" | "-de" => pusher(&self.desktop, info),
            "--term" | "-t" => pusher(&self.term, info),
            "--session" | "-ds" => pusher(&self.session, info),
            "--kernel" | "-k" => pusher(&self.kernel, info),
            "--host_device" | "-hd" => pusher(&self.host_device, info),
            "--uptime" | "-u" => pusher(&self.uptime, info),
            "--cpu" | "-c" => pusher(&self.cpu, info),
            "--gpu" | "-g" => pusher(&self.gpu, info),
            "--memory" | "-m" => pusher(&self.mem, info),
            "--help" | "-h" => self.print_exit(HELP),
            _ => self.print_exit(&format!("'{}' not a valid argument", arg)),
        }

        Ok(())
    }

    fn print_exit(&self, arg: &str) {
        println!("{}", arg);
        process::exit(0)
    }

    fn print_faces(&self, info: &Vec<String>) {
        let faces_array = [
            ("( ͡° ͜ʖ ͡°)", 3),
            ("(⌐■_■)", 4),
            (r"¯\_ツ_/¯", 3),
            ("(ಠ_ಠ)", 5),
            ("༼ つ ◕_◕ ༽つ  ", 2),
            ("(ノಠ益ಠ)ノ彡┻━┻", 2),
            ("(╯°□°）╯︵ ┻━┻", 2),
            ("(◕‿◕✿)", 4),
        ];

        let face = faces_array.choose(&mut rand::thread_rng());
        let face_len = face.unwrap().0.repeat(face.unwrap().1);

        for line in info {
            println!("{}\t{}", face_len , line)
        }
    }
}


const HELP: &str = "
Usage: lenny-fetch [FLAG]\n
FLAGS:
\t-a\t\tView system architecture
\t-b\t\tView system board family
\t-c\t\tView system CPU
\t-d\t\tView desktop environment
\t-D\t\tView Linux Distribution
\t-h, --help\tView this help information
\t-H\t\tView current user home directory
\t-k\t\tView system kernel
\t-m\t\tView system memory
\t-n\t\tView system host name
\t-o\t\tView system OS
\t-s\t\tView user shell
\t-S\t\tView current graphics session
";
