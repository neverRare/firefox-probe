use firefox_probe::firefox_profile;
use regex::Regex;
use sqlite::{Value, open};
use std::{
    collections::{BinaryHeap, HashMap},
    fs::exists,
};

#[derive(Debug, Clone, Copy)]
struct Rank<T, U> {
    value: T,
    rank: U,
}
impl<T, U> PartialEq for Rank<T, U>
where
    U: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}
impl<T, U> Eq for Rank<T, U> where U: Eq {}
impl<T, U> PartialOrd for Rank<T, U>
where
    U: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}
impl<T, U> Ord for Rank<T, U>
where
    U: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}
fn main() {
    let domain_regex = Regex::new("^https?://[^/]+/").unwrap();
    let mut path = match firefox_profile() {
        Some(path) => path,
        None => {
            println!("Firefox not found");
            return;
        }
    };
    path.push("places.sqlite");
    assert!(exists(&path).unwrap());
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
