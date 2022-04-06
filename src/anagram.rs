pub struct AnagramGroup<'a> {
    pub count: usize,
    pub words: Vec<&'a str>
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