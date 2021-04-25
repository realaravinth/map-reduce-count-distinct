use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Map<'a> {
    pub map: HashMap<&'a str, usize>,
}

impl<'a> Map<'a> {
    pub fn new() -> Self {
        Map {
            map: HashMap::new(),
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
}
