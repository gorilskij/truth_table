use indexmap::map::IndexMap;

pub struct VarValues(IndexMap<String, bool>);

impl VarValues {
    pub fn new(names: &[String]) -> Self {
        let mut map = IndexMap::new();
        for name in names.iter().map(Clone::clone) {
            map.entry(name).or_insert(true);
        }
        VarValues(map)
    }

    pub fn get_value<S: ToString>(&self, name: S) -> bool {
        *self.0.get(&name.to_string()).unwrap_or_else(||
            panic!("tried to get value of nonexistent variable '{}'", &name.to_string()))
    }

    pub fn names(&self) -> impl Iterator<Item=&String> {
        self.0.keys()
    }

    pub fn values(&self) -> Vec<bool> {
        self.0.values().copied().collect()
    }

    pub fn advance(&mut self) -> bool {
        self.0.values_mut().rev().any(|value| {
            *value = !*value;
            !*value
        })
    }
}