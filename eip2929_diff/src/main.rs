use anyhow::Result;
use regex::Regex;
use std::{collections::HashMap, fs::File, io::BufRead, path::Path};

const CAPTURE: &str =
    r#"([A-Za-z0-9/._]*): post state root mismatch: got ([0-9a-f]{64}), want ([0-9a-f]{64})"#;

fn read_file(path: &std::path::Path) -> Result<HashMap<String, (String, String)>> {
    let mut h = HashMap::new();
    let re = Regex::new(CAPTURE)?;
    let file = File::open(path)?;
    for line in std::io::BufReader::new(file).lines() {
        let line = line?;
        let mut captures = re.captures_iter(&line);
        let capture_1 = captures.next().expect("cannot parse line");
        let name = capture_1.get(1).unwrap().as_str().to_string();
        let got = capture_1.get(2).unwrap().as_str().to_string();
        let want = capture_1.get(3).unwrap().as_str().to_string();
        h.insert(name, (got, want));
    }
    Ok(h)
}

fn diff<T, I1, I2>(first_it: I1, second_it: I2) -> (Vec<T>, Vec<T>, Vec<T>)
where
    T: PartialOrd + Ord + Copy,
    I1: IntoIterator<Item = T>,
    I2: IntoIterator<Item = T>,
{
    let mut both = Vec::new();
    let mut only_in_first = Vec::new();
    let mut only_in_second = Vec::new();

    let mut first = first_it.into_iter().collect::<Vec<_>>();
    let mut second = second_it.into_iter().collect::<Vec<_>>();
    first.sort();
    second.sort();
    let mut first_it = first.iter();
    let mut second_it = second.iter();

    let mut maybe_first = first_it.next();
    let mut maybe_second = second_it.next();
    loop {
        match (maybe_first.is_some(), maybe_second.is_some()) {
            (true, true) => {
                let first = maybe_first.unwrap();
                let second = maybe_second.unwrap();
                match first.cmp(second) {
                    std::cmp::Ordering::Equal => {
                        both.push(*first);
                        maybe_first = first_it.next();
                        maybe_second = second_it.next();
                    }
                    std::cmp::Ordering::Greater => {
                        only_in_second.push(*second);
                        maybe_second = second_it.next();
                    }
                    std::cmp::Ordering::Less => {
                        only_in_first.push(*first);
                        maybe_first = first_it.next();
                    }
                }
            }
            (true, false) => {
                only_in_first.push(*maybe_first.unwrap());
                maybe_first = first_it.next();
            }
            (false, true) => {
                only_in_second.push(*maybe_second.unwrap());
                maybe_second = second_it.next();
            }
            (false, false) => {
                break;
            }
        }
    }
    (both, only_in_first, only_in_second)
}

fn main() -> Result<()> {
    let oe_h = read_file(&Path::new("./log_oe"))?;
    let geth_h = read_file(&Path::new("./log_geth"))?;

    let mut ok_count = 0;
    let mut fail_count = 0;
    let mut root_mismatch_count = 0;

    let (both, only_oe, only_geth) = diff(oe_h.keys(), geth_h.keys());
    for test_name in both.into_iter() {
        let oe_got_want = oe_h.get(test_name).unwrap();
        let geth_got_want = geth_h.get(test_name).unwrap();
        if oe_got_want.1 == geth_got_want.1 {
            if oe_got_want.0 == geth_got_want.0 {
                ok_count += 1;
            } else {
                fail_count += 1;
                println!(
                    "Failed {} OE:{} Geth:{}",
                    test_name, oe_got_want.0, geth_got_want.0
                )
            }
        } else {
            root_mismatch_count += 1;
        }
    }
    let disjoint_only_in_oe_count = only_oe.len();
    let disjoint_only_in_geth_count = only_geth.len();

    println!("ok_count={}", ok_count);
    println!("fail_count={}", fail_count);
    println!("disjoint_only_in_oe_count={}", disjoint_only_in_oe_count);
    println!(
        "disjoint_only_in_geth_count={}",
        disjoint_only_in_geth_count
    );
    println!("root_mismatch_count={}", root_mismatch_count);

    Ok(())
}

#[test]
fn test_diff() {
    let empty: Vec<&str> = Vec::new();
    assert_eq!(
        diff(&["a", "b", "c"], &["a", "b", "c"]),
        (vec![&"a", &"b", &"c"], vec![], vec![])
    );
    assert_eq!(
        diff(&["a", "b"], &["b", "c"]),
        (vec![&"b"], vec![&"a"], vec![&"c"])
    );
    assert_eq!(diff(&["a"], &empty), (vec![], vec![&"a"], vec![]));
    assert_eq!(diff(&empty, &["b"]), (vec![], vec![], vec![&"b"]));
}
