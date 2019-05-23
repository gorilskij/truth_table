use std::collections::LinkedList;

pub struct VarValues(LinkedList<(String, bool)>);

impl VarValues {
    pub fn new(names: &Vec<String>) -> Self {
        let mut list = LinkedList::new();
        for name in names {
            list.push_back((name.clone(), true))
        }
        VarValues(list)
    }

    pub fn get_value(&self, name: &String) -> bool {
        match self.0.iter().find(|x| *x.0 == *name) {
            None => panic!("tried to get value of variable with inexistent name '{}'", name),
            Some((_, value)) => *value
        }
    }

    pub fn names(&self) -> Vec<&String> {
        self.0.iter().map(|x| &x.0).collect()
    }

    pub fn values(&self) -> Vec<bool> {
        self.0.iter().map(|x| x.1).collect()
    }

    pub fn advance(&mut self) -> bool {
        for entry in self.0.iter_mut().rev() {
            let value = entry.1;
            entry.1 = !value;
            if value { return true }
        }
        false
    }
}