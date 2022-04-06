mod anagram;
mod algos;
mod misc;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use crate::algos::*;
use crate::misc::*;
use crate::anagram::AnagramGroup;

pub struct GameInfo {
    name: String,
    platform: String,
    release_year: i32,
    genre: String,
    publisher: String,
    na_sales: f32,
    eu_sales: f32,
    jp_sales: f32,
    other_sales: f32,
    global_sales: f32,
    critic_score: Option<f32>,
    critic_count: Option<f32>,
    user_score: Option<f32>,
    user_count: Option<f32>,
    developer: Option<String>,
    rating: Option<String>
}

fn main() {
    /*let mut file = File::open(
        std::env::args()
            .skip(1)
            .next()
            .unwrap()
    ).unwrap();

    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let text_unchanged = text.clone();
    let text = text.to_lowercase();

    task1(&text);

    task2(&text_unchanged);

    task3(&text_unchanged);*/

    let mut file = File::open("Video_Games.csv").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf);
    let games = parse(&buf);
}

fn task1(text: &String) {
    let w = to_words(text).collect::<Vec<&str>>();
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

    println!("Found in {}ms\n", (end - begin).as_millis());
}

fn task2(text: &String) {
    let w = to_words(text).collect::<Vec<&str>>();
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

    println!("Found in {}ms\n", (end - begin).as_millis());
}

fn task3(text: &String) {
    let w = to_words(text)
        .filter(|x|
            !x.chars()
                .all(|x| x.is_digit(10))
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