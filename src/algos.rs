use crate::anagram::AnagramGroup;
use std::collections::HashMap;
use std::ops::Deref;

pub fn qsort<T: Copy, C: Copy + Fn(T, T) -> bool>(vec: &mut Vec<T>, begin: usize, end: usize, cmp: C) {
    if end - begin <= 1 {
        return;
    }
    let p = partition(vec, begin, end, cmp);

    qsort(vec, begin, p, cmp);
    qsort(vec, p + 1, end, cmp);
}

fn partition<T: Copy, C: Fn(T, T) -> bool>(vec: &mut Vec<T>, begin: usize, end: usize, cmp: C) -> usize {
    let mid_elem = vec[(end + begin) / 2];
    let mut i = begin;
    let mut j = end;
    loop {
        while cmp(mid_elem, vec[i]) {
            i += 1;
        }
        while cmp(vec[j], mid_elem) {
            j -= 1;
        }

        if i >= j {
            return j;
        }

        vec.swap(i, j);
        i += 1;
        j -= 1;
    }
}

pub fn qsort2(vec: &mut Vec<(String, AnagramGroup)>, begin: usize, end: usize) {
    if end - begin <= 1 {
        return;
    }
    let p = partition2(vec, begin, end);

    qsort2(vec, begin, p);
    qsort2(vec, p + 1, end);
}

fn partition2(vec: &mut Vec<(String, AnagramGroup)>, begin: usize, end: usize) -> usize {
    let mid_elem = vec[(end + begin) / 2].1.count;
    let mut i = begin;
    let mut j = end;
    loop {
        while mid_elem < vec[i].1.count {
            i += 1;
        }

        while vec[j].1.count < mid_elem {
            j -= 1;
        }

        if i >= j {
            return j;
        }

        vec.swap(i, j);
        i += 1;
        j -= 1;
    }
}

pub fn sort_hashmap<K, V: PartialOrd>(map: &HashMap<K, V>) -> Vec<(&K, &V)> {
    let mut w = map.iter().collect::<Vec<(&K, &V)>>();
    let len = w.len();
    qsort(&mut w, 0, len - 1, |x, y| x.1 < y.1);

    w
        .into_iter()
        .map(|(x, y)| (x, y))
        .collect()
}