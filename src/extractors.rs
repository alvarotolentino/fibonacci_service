use regex::Regex;
use spin_sdk::http::Request;

pub fn extract_base_number(req: &Request) -> usize {
    let path = req.uri().path().to_string();
    let re = Regex::new(r"/fibonacci/(\d+)").unwrap();
    let mut cap = re.captures_iter(&path);
    match cap.next() {
        Some(c) => {
            let re = &c[1];
            re.parse().unwrap_or(0)
        }
        None => 0,
    }
}
