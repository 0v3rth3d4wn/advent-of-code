fn main() {
    let input = "ckczppom";

    let mut i = 0;
    loop {
        let mut res = String::from(input);
        res.push_str(&i.to_string());
        let res_md5 = md5::compute(res);
        if res_md5.starts_with(b"00000") {
            println!("{:?}", res_md5);
            println!("{}", i);
            break;
        }

        i += 1;
    }
}
