#[derive(Debug)]
struct Element<T> {
    /// Preferences here is a Vec<usize> containing the indices of the preferences of the element, from most preferred to least preferreds
    preferences: Vec<usize>,
    value: T,
}
impl<T> Element<T> {
    fn new(value: T) -> Self {
        Element {
            preferences: vec![],
            value,
        }
    }
    fn set_preferences(&mut self, preferences: Vec<usize>) {
        self.preferences = preferences;
    }
}

fn generate_test_set(n: usize) -> Vec<Element<usize>> {
    let mut set: Vec<Element<usize>> = vec![];
    for i in 0..n {
        set.push(Element::new(i));
    }
    set
}

fn main() {
    let A = generate_test_set(10);
    let B = generate_test_set(10);
    A.
}
