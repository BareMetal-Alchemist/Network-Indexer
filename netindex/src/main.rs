
// Function for counting bits that are 1s

fn count_ones(mut n: u8) -> u32 {
    let mut count = 0;

    while n != 0 {
        n &= n - 1;
        count += 1;
    }
    count
}


fn main() {
    let  ip: String = String::from("123.115.5.123");
    let  subnet: String = String::from("255.255.252.0");
    
    let  ipv4: Vec<u8> = ip.split('.').map(|s| s.parse::<u8>().unwrap()).collect();
    let  subnet_vec: Vec<u8> = subnet.split('.').map(|s| s.parse::<u8>().unwrap()).collect();
    let mut start: String = String::new();

    println!("{:?}", ipv4);
    println!("{:?}", subnet_vec);
    
    for i in 0..4 {
        start += &(ipv4[i] & subnet_vec[i]).to_string();
        start += ".";

    }

    println!("{start}");

    let mut count2: u32 = 0;

    for n in  subnet_vec {
        count2 += count_ones(n);
    }



    println!("Your network block is {start}/{count2}");
}
