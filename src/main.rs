use firefox_probe::{Rank, firefox_places};
use regex::Regex;
use sqlite::{Value, open};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let domain_regex = Regex::new("^https?://[^/]+/").unwrap();
    let places = firefox_places().expect("Firefox not found");
    let mut map = HashMap::new();
    for places in places {
        let connection = open(places).unwrap();
        let rows = connection
            .prepare("SELECT url, visit_count FROM moz_places")
            .unwrap();
        for row in rows {
            let mut row = row.unwrap();
            let Value::String(url) = row.take("url") else {
                panic!("url is not string");
            };
            let domain: Box<str> = match domain_regex.find(&url) {
                Some(domain) => domain.as_str().into(),
                None => continue,
            };
            let Value::Integer(count) = row.take("visit_count") else {
                panic!("visit_count is not string;")
            };
            let rank: &mut u64 = map.entry(domain).or_default();
            *rank += count as u64;
        }
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
