// Function for counting bits that are 1s

fn count_ones(mut n: u8) -> u8 {
    let mut count = 0;

    while n != 0 {
        n &= n - 1;
        count += 1;
    }
    count
}

fn find_cidr(subnet: &Vec<u8>) -> String {
    let mut count: u8 = 0;

    for &n in subnet {
        count += count_ones(n);
    }

    let cidr: String = count.to_string();
    cidr
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
    netaddr
}

fn main() {
    let ip: String = String::from("123.115.5.123");
    let subnet: String = String::from("255.255.252.0");

    let netaddr: String = find_netaddr(ip, subnet);

    println!("{}", netaddr);
}
