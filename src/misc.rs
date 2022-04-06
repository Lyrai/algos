use std::collections::HashMap;
use crate::GameInfo;

pub fn to_words(str: &String) -> impl Iterator<Item = &str>{
    str
        .split(|x: char| x.is_whitespace() || x.is_ascii_punctuation() || x == '–')
        .filter(|x| !x.is_empty())
}

pub fn insert_if_name<'a>(map: &mut HashMap<&'a str, usize>, item: &'a str) {
    if item.starts_with(|x: char| x.is_ascii_uppercase()) {
        map.insert(item, *map.get(item).unwrap_or(&0) + 1);
    }
}

pub fn as_anagram(str: &str) -> String {
    let mut chars = str.chars().collect::<Vec<char>>();
    let chars = chars.as_mut_slice();
    chars.sort();
    String::from_iter(chars.into_iter().map(|x| *x))
}

pub fn parse(text: &String) -> Vec<GameInfo> {
    let mut result = vec![];
    let lines = text.split('\n').collect::<Vec<&str>>();
    for line in lines.iter().skip(1) {
        let parameters = line.split(',').collect::<Vec<&str>>();
        let item = GameInfo {
            name: parameters[0].to_string(),
            platform: parameters[1].to_string(),
            release_year: parameters[2].parse().unwrap_or_default(),
            genre: parameters[3].to_string(),
            publisher: parameters[4].to_string(),
            na_sales: parameters[5].parse().unwrap(),
            eu_sales: parameters[6].parse().unwrap(),
            jp_sales: parameters[7].parse().unwrap(),
            other_sales: parameters[8].parse().unwrap(),
            global_sales: parameters[9].parse().unwrap(),
            critic_score: if parameters[10].is_empty() {None} else {Some(parameters[10].parse().unwrap_or_default())},
            critic_count: if parameters[11].is_empty() {None} else {Some(parameters[11].parse().unwrap_or_default())},
            user_score: if parameters[12].is_empty() {None} else {Some(parameters[12].parse().unwrap_or_default())},
            user_count: if parameters[13].is_empty() {None} else {Some(parameters[13].parse().unwrap_or_default())},
            developer: if parameters[14].is_empty() {None} else {Some(parameters[14].to_string())},
            rating: if parameters[15].is_empty() {None} else {Some(parameters[15].to_string())}
        };
        result.push(item);
    }

    result
}