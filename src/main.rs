mod anagram;
mod algos;
mod misc;

use std::borrow::Borrow;
use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use crate::algos::*;
use crate::misc::*;
use crate::anagram::AnagramGroup;

#[derive(Clone)]
pub struct GameInfo {
    name: String,
    platform: String,
    release_year: i32,
    genre: String,
    publisher: String,
    na_sales: f64,
    eu_sales: f64,
    jp_sales: f64,
    other_sales: f64,
    global_sales: f64,
    critic_score: Option<f64>,
    critic_count: Option<f64>,
    user_score: Option<f64>,
    user_count: Option<f64>,
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

    task4(games.to_vec());

    task5(games.to_vec());

    task6(games.to_vec());
}

fn task1(text: &String) {
    let w = to_words(text).collect::<Vec<&str>>();
    println!("Text contains {} words", w.len());

    let mut words = HashMap::with_capacity(w.len());
    let begin = Instant::now();

    for word in w {
        words.insert(word, *words.get(word).unwrap_or(&0) + 1);
    }

    let w = sort_hashmap(&words);

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

    let w = sort_hashmap(&words);

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

fn task4(games: Vec<GameInfo>) {
    let begin = Instant::now();
    let mut publishers: HashMap<String, HashMap<String, Vec<GameInfo>>> = HashMap::new();
    for game in games {
        match publishers.get_mut(&game.publisher) {
            Some(genre) => {
                match genre.get_mut(&game.genre) {
                    Some(games) => games.push(game),
                    None => { genre.insert(game.genre.clone(), vec![game]); }
                }
            }
            None => {
                let mut map = HashMap::new();
                let publisher = game.publisher.clone();
                map.insert(game.genre.clone(), vec![game]);
                publishers.insert(publisher, map);
            }
        }
    }

    for (publisher, genres) in publishers {
        println!("Publisher {}:", publisher);
        for (genre, games) in genres {
            let mut sum = 0f64;
            for game in games {
                sum += game.global_sales;
            }

            println!("  {}: {} total sales", genre, sum);
        }
    }
    let end = Instant::now();
    println!("Found in {}ms\n", (end - begin).as_millis());
}

fn task5(games: Vec<GameInfo>) {
    let mut years: HashMap<i32, HashMap<String, f64>> = HashMap::new();

    let begin = Instant::now();
    for game in games {
        match years.get_mut(&game.release_year) {
            Some(genre) => {
                match genre.get_mut(&game.genre) {
                    Some(revenue) => *revenue += game.global_sales,
                    None => { genre.insert(game.genre.clone(), game.global_sales); }
                }
            }
            None => {
                let mut map = HashMap::new();
                map.insert(game.genre.clone(), game.global_sales);
                years.insert(game.release_year, map);
            }
        }
    }

    for (year, genres) in years {
        let mut max = 0f64;
        let mut res_genre = &String::new();
        for (genre, sum) in &genres {
            if *sum > max {
                max = *sum;
                res_genre = genre;
            }
        }

        println!("{}: {} in {}", year, max, res_genre);
    }

    let end = Instant::now();
    println!("Found in {}ms\n", (end - begin).as_millis());
}

fn task6(games: Vec<GameInfo>) {
    let mut publishers: HashMap<String, HashMap<String, f64>> = HashMap::new();

    let begin = Instant::now();
    for game in games {
        match publishers.get_mut(&game.publisher) {
            Some(genre) => {
                if let Some(developer) = &game.developer {
                    match genre.get_mut(developer.as_str()) {
                        Some(revenue) => *revenue += game.global_sales,
                        None => { genre.insert(developer.clone(), game.global_sales); }
                    }
                }
            }
            None => {
                if let Some(developer) = &game.developer {
                    let mut map = HashMap::new();
                    let publisher = game.publisher;
                    map.insert(developer.clone(), game.global_sales);
                    publishers.insert(publisher, map);
                }
            }
        }
    }

    for (publisher, developers) in publishers {
        let developers = sort_hashmap(&developers);
        println!("{}: ", publisher);
        for i in 0..min(developers.len(), 5) {
            println!("  {} from {}", developers[i].1, developers[i].0);
        }
    }

    let end = Instant::now();
    println!("Found in {}ms\n", (end - begin).as_millis());
}