// Function for counting bits that are 1s
use std::process::Command;

fn count_ones(mut n: u8) -> u8 {
    let mut count = 0;

    while n != 0 {
        n &= n - 1;
        count += 1;
    }
    count // return count
}

fn find_cidr(subnet: &Vec<u8>) -> String {
    let mut count: u8 = 0;

    for &n in subnet {
        count += count_ones(n);
    }

    let cidr: String = count.to_string();
    cidr //return cidr
}

fn find_netaddr(ip: String, subnet: String) -> String {
    let mut netaddr: String = String::new();

    let ipv4: Vec<u8> = ip.split('.').map(|s| s.parse::<u8>().unwrap()).collect();
    let subnet_vec: Vec<u8> = subnet
        .split('.')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let cidr: String = find_cidr(&subnet_vec);

    for i in 0..4 {
        netaddr += &(ipv4[i] & subnet_vec[i]).to_string();
        if i == 3 {
            break;
        }
        netaddr += ".";
    }
    netaddr += &cidr;
    netaddr // return netaddr
}

fn get_ip() -> anyhow::Result<()> {
    let out = Command::new("ifconfig").arg("-a").output()?;
    if !out.status.success() {
        anyhow::bail!("ifconfig failed: {}", String::from_utf8_lossy(&out.stderr));
    }

    let text = String::from_utf8_lossy(&out.stdout);

    for line in text.lines() {
        if line.contains("inet") || line.contains("inet addr:") {
            if let Some(ip) = line
                .split_whitespace()
                .find(|tok| {
                    tok.chars()
                        .next()
                        .map(|c| c.is_ascii_digit())
                        .unwrap_or(false)
                        && tok.chars().filter(|&c| c == '.').count() == 3
                })
                .map(|tok| tok.trim_start_matches("addr:"))
            {
                println!("ipv4: {ip}");
            }
        }

        if line.contains("netmask") || line.contains("Mask:") {
            let mut parts = line.split_whitespace();
            let mut nm: Option<&str> = None;

            while let Some(tok) = parts.next() {
                if tok == "netmask" {
                    nm = parts.next();
                    break;
                } else if let Some(rest) = tok.strip_prefix("Mask:") {
                    nm = Some(rest);
                    break;
                }
            }

            if let Some(nm) = nm {
                println!("netmask: {nm}");
            }
        }
    }

    Ok(())
}

fn main() {
    match get_ip() {
        Ok(()) => println!("success"),
        Err(e) => eprintln!("Error: {e}"),
    }

    let ip: String = String::from("123.115.5.123");
    let subnet: String = String::from("255.255.252.0");

    let netaddr: String = find_netaddr(ip, subnet);

    println!("{}", netaddr);
}
