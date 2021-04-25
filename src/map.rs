use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Map<'a> {
    //pub map: HashMap<&'a str, usize>,
    pub map: WordCount<'a>,
}

impl<'a> Map<'a> {
    pub fn new() -> Self {
        Map {
            map: WordCount::new(),
        }
    }

    pub fn insert(&mut self, word: &'a str) {
        if self.map.contains_key(word) {
            let val = self.map.get_mut(word).unwrap();
            *val += 1;
        } else {
            self.map.insert(word, 1);
        }
    }

    pub fn display(&self) {
        println!("{:#?}", self.map);
    }

    pub fn merge(master: WordCount<'a>, hash: WordCount<'a>) -> WordCount<'a> {
        master.into_iter().chain(hash).collect()
    }
}

type WordCount<'a> = HashMap<&'a str, usize>;
