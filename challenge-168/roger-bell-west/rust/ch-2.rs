#! /bin/sh
//usr/bin/env rustc --test $0 -o ${0}x && ./${0}x --nocapture; rm -f ${0}x ; exit

use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;
use itertools::Itertools;

#[test]
fn test_ex1() {
    assert_eq!(homeprime(10), 773);
}

#[test]
fn test_ex2() {
    assert_eq!(homeprime(16), 31636373);
}

fn genprimes(mx: usize) -> Vec<usize> {
    let mut primesh: HashSet<usize> = HashSet::from_iter(2..=3);
    for i in (6..=mx).step_by(6) {
        for j in [i - 1, i + 1] {
            if j < mx {
                primesh.insert(j);
            }
        }
    }
    let mut q = VecDeque::from([2, 3, 5, 7]);
    let mut p = q.pop_front().unwrap();
    let mr = (mx as f64).sqrt() as usize;
    while p <= mr {
        if primesh.contains(&p) {
            for i in (p * p..=mx).step_by(p as usize) {
                primesh.remove(&i);
            }
        }
        if q.len() < 2 {
            let t = q[0] + 4;
            q.push_back(t);
            q.push_back(t + 2);
        }
        p = q.pop_front().unwrap();
    }
    let mut primes = primesh.iter().map(|i| *i).collect::<Vec<usize>>();
    primes.sort();
    primes
}

fn primefactor(n: usize) -> HashMap<usize, usize> {
    let mut f: HashMap<usize, usize> = HashMap::new();
    let mut m = n;
    for p in genprimes(1+(n as f64).sqrt() as usize).iter() {
        while m % p == 0 {
            m /= p;
            let en = f.entry(*p).or_insert(0);
            *en += 1;
        }
        if m == 1 {
            break;
        }
    }
    if m > 1 {
        let en = f.entry(m).or_insert(0);
        *en += 1;
    }
    f
}

fn homeprime(n0: usize) -> usize {
    let mut n=n0;
    loop {
        let t = primefactor(n);
        println!("{:?} {:?}",n,t);
        if t.len() == 1 && *t.values().max().unwrap() == 1 {
            break;
        }
        let mut ns: String = String::new();
        for d in t.keys().sorted() {
            let ts = d.to_string();
            for _ in 0..*t.get(&d).unwrap() {
                ns.push_str(&ts);
            }
        }
        n = ns.parse::<usize>().unwrap();
    }
    n
}
