use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

struct AnagramGroup<'a> {
    count: usize,
    words: Vec<&'a str>
}

impl<'a> AnagramGroup<'a> {
    pub fn new(word: &'a str) -> Self {
        AnagramGroup {
            count: 1,
            words: vec![word]
        }
    }

    pub fn push(&mut self, word: &'a str) {
        if !self.words.contains(&word) {
            self.words.push(word);
            self.count += 1;
        }
    }
}

fn qsort<T: Copy, C: Copy + Fn(T, T) -> bool>(vec: &mut Vec<T>, begin: usize, end: usize, cmp: C) {
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

fn qsort2(vec: &mut Vec<(String, AnagramGroup)>, begin: usize, end: usize) {
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

fn to_words(str: &String) -> impl Iterator<Item = &str>{
    str
        .split(|x: char| x.is_whitespace() || x.is_ascii_punctuation() || x == '–')
        .filter(|x| !x.is_empty())
}

fn insert_if_name<'a>(map: &mut HashMap<&'a str, usize>, item: &'a str) {
    if item.starts_with(|x: char| x.is_ascii_uppercase()) {
        map.insert(item, *map.get(item).unwrap_or(&0) + 1);
    }
}

fn sort_hashmap(map: HashMap<&str, usize>) -> Vec<(&str, usize)> {
    let mut w = map.into_iter().collect::<Vec<(&str, usize)>>();
    let len = w.len();
    qsort(&mut w, 0, len - 1, |x, y| x.1 < y.1);

    w
}

fn as_anagram(str: &str) -> String {
    let mut chars = str.chars().collect::<Vec<char>>();
    let chars = chars.as_mut_slice();
    chars.sort();
    String::from_iter(chars.into_iter().map(|x| *x))
}

fn main() {
    let mut file = File::open(
        std::env::args()
            .skip(1)
            .next()
            .unwrap()
    ).unwrap();

    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let text_unchanged = text.clone();
    let text = text.to_lowercase();

    let w = to_words(&text).collect::<Vec<&str>>();

    println!("Text contains {} words", w.len());

    let mut words = HashMap::with_capacity(w.len());
    let begin = Instant::now();

    for word in w {
        words.insert(word, *words.get(word).unwrap_or(&0) + 1);
    }

    let w = sort_hashmap(words);

    let end = Instant::now();
    for (word, count) in w.into_iter().take(40) {
        println!("{} {}", word, count);
    }

    println!("Found in {}ms", (end - begin).as_millis());

    let w = to_words(&text_unchanged).collect::<Vec<&str>>();
    let mut words = HashMap::with_capacity(w.len());

    let begin = Instant::now();
    for word in w {
        insert_if_name(&mut words, word);
    }

    let w = sort_hashmap(words);

    let end = Instant::now();
    for (word, count) in w.iter().take(20) {
        println!("{} {}", word, count);
    }

    println!("Found in {}ms", (end - begin).as_millis());

    let w = to_words(&text_unchanged)
        .filter(|x|
            !x.chars()
                .all(|x|
                    x.is_digit(10)
                )
        ).collect::<Vec<&str>>();
    let mut words: HashMap<String, AnagramGroup> = HashMap::with_capacity(w.len());
    let begin = Instant::now();
    for word in w {
        let anagram = as_anagram(word);
        match words.get_mut(&anagram) {
            Some(group) => group.push(word),
            None => { words.insert(anagram, AnagramGroup::new(word)); },
        }
    }

    let mut w = words.into_iter().collect::<Vec<(String, AnagramGroup)>>();
    let len = w.len();
    qsort2(&mut w, 0, len - 1);
    let end = Instant::now();

    for (word, AnagramGroup {count, words}) in w.iter().take(10) {
        println!("{} {} {:?}", word, count, words);
    }
    println!("Found in {}ms", (end - begin).as_millis());
}