use firefox_probe::{Rank, firefox_places};
use regex::Regex;
use sqlite::{Value, open};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let domain_regex = Regex::new("^https?://[^/]+/").unwrap();
    let path = match firefox_places() {
        Some(path) => path.into_iter().next().unwrap(),
        None => {
            println!("Firefox not found");
            return;
        }
    };
    let connection = open(path).unwrap();
    let rows = connection
        .prepare("SELECT url, visit_count FROM moz_places")
        .unwrap();
    let mut map = HashMap::new();
    for row in rows {
        let mut row = row.unwrap();
        let url = match row.take("url") {
            Value::String(url) => url,
            _ => panic!("url is not string"),
        };
        let domain: Box<str> = match domain_regex.find(&url) {
            Some(domain) => domain.as_str().into(),
            None => continue,
        };
        let count = match row.take("visit_count") {
            Value::Integer(count) => count as u64,
            _ => panic!("visit_count is not integer"),
        };
        let rank: &mut u64 = map.entry(domain).or_default();
        *rank += count;
    }
    let mut entries: BinaryHeap<_> = map
        .into_iter()
        .map(|(value, rank)| Rank { value, rank })
        .collect();
    for _ in 0..20 {
        let rank = match entries.pop() {
            Some(rank) => rank,
            None => break,
        };
        println!("{:30} {:>5}", rank.value, rank.rank);
    }
}
