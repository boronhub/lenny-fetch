use std::collections::HashMap;
use std::env::var;
use std::process::{Command, Stdio};
use crate::utils::{stdout_str, parse_split, read_lines};

pub struct Info {
    pub shell: (String, Option<String>),
    pub desktop: (String, Option<String>),
    pub term: (String, Option<String>),
    pub session: (String, Option<String>),
    pub kernel: (String, Option<String>),
    pub distro: (String, Option<String>),
    pub cpu: (String, Option<String>),
    pub mem: (String, Option<String>),
    pub uptime: (String, Option<String>),
    pub gpu: (String, Option<String>),
    pub host_device: (String, Option<String>),
}

impl Info {
    pub fn new() -> Self {
        Self {
            distro: (String::from("Distro:\t"), Self::get_distro()),
            shell: (String::from("Shell:\t"), Self::from_env("SHELL")),
            desktop: (
                String::from("DE/WM:\t"),
                Self::from_env("XDG_SESSION_DESKTOP"),
            ),
            term: (String::from("Terminal: "), Self::from_env("TERM")),
            session: (
                String::from("Session: "),
                Self::from_env("XDG_SESSION_TYPE"),
            ),
            kernel: (String::from("Kernel:\t"), Self::get_cmd("uname", "sr")),
            host_device: (String::from("Host Device: "), Self::get_host_device()),
            cpu: (String::from("CPU:\t"), Self::get_cpu()),
            uptime: (String::from("Uptime:\t"), Self::get_cmd("uptime", "p")),
            gpu: (String::from("GPU:\t"), Self::get_gpu()),
            mem: (String::from("Memory:\t"), Self::get_mem()),
        }
    }

    pub fn from_env(s: &str) -> Option<String> {
        var(s).ok()
    }

    pub fn get_cmd(cmd: &str, arg: &str) -> Option<String> {
        let output = Command::new(cmd).arg("-".to_owned() + arg).output().ok()?;
        return Some(stdout_str(&output.stdout));
    }

    fn get_host_device() -> Option<String> {
        if let Some(mut lines) = read_lines("/sys/devices/virtual/dmi/id/product_name").ok() {
            return Some(lines.next().unwrap().ok()?);
        }
        None
    }

    fn get_distro() -> Option<String> {
        if let Some(lines) = read_lines("/etc/os-release").ok() {
            for line in lines {
                let txt = line.ok().unwrap();
                if txt.starts_with("PRETTY_NAME") {
                    let mut tmp_name = parse_split(txt, '=', 1);
                    tmp_name.remove(0);
                    tmp_name.remove(tmp_name.len() - 1);
                    return Some(tmp_name);
                }
            }
        }
        None
    }

    fn get_gpu() -> Option<String> {
        let mut lspci_out_child = Command::new("lspci").stdout(Stdio::piped()).spawn().ok()?;

        if let Some(lspci_out) = lspci_out_child.stdout.take() {
            let mut grep_out_child = Command::new("grep")
                .arg("VGA")
                .stderr(Stdio::null())
                .stdin(lspci_out)
                .stdout(Stdio::piped())
                .spawn()
                .ok()?;

            lspci_out_child.wait().ok()?;

            if let Some(grep_out) = grep_out_child.stdout.take() {
                let cut_out_child = Command::new("cut")
                    .args(&["-d", ":", "-f3"])
                    .stderr(Stdio::null())
                    .stdin(grep_out)
                    .stdout(Stdio::piped())
                    .spawn()
                    .ok()?;

                let head_stdout = cut_out_child.wait_with_output().ok()?;
                grep_out_child.wait().ok()?;
                let output = String::from_utf8(head_stdout.stdout)
                    .unwrap()
                    .trim()
                    .to_string();
                if output.is_empty() {
                    return None;
                }
                return Some(output);
            }
        }

        None
    }

    fn get_cpu() -> Option<String> {
        if let Some(lines) = read_lines("/proc/cpuinfo").ok() {
            for line in lines {
                let txt = line.ok().unwrap();
                if txt.starts_with("model name") {
                    let tmp_name = parse_split(txt, ':', 1);
                    return Some(parse_split(tmp_name, '@', 0));
                }
            }
        }
        None
    }

    fn get_mem() -> Option<String> {
        let vars = vec!["MemTotal", "MemFree", "Buffers", "Cached"];
        let mut vals = vec![];
        if let Some(lines) = read_lines("/proc/meminfo").ok() {
            for line in lines {
                let txt = line.ok().unwrap();
                for x in &vars {
                    if txt.starts_with(x) {
                        let tmp = parse_split(txt, ':', 1);
                        let val: u32 = parse_split(tmp, ' ', 0).parse().ok()?;
                        vals.push(val);
                        break;
                    }
                }
            }
            let meminfo: HashMap<_, _> = vars.into_iter().zip(vals.into_iter()).collect();
            return Some(format!(
                "{}/{} MiB",
                (meminfo.get("MemTotal")?
                    - (meminfo.get("MemFree")?
                        + meminfo.get("Buffers")?
                        + meminfo.get("Cached")?))
                    / 1024,
                meminfo.get("MemTotal")? / 1024
            ));
        }
        None
    }
}
