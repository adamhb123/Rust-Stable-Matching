use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
#[derive(Clone)]
struct Element<T> {
    index: usize,
    partner_index: usize,
    value: T,
    /// Preferences here is a Vec<usize> containing the indices of the preferences of the element, from most preferred to least preferreds
    preferences: Vec<usize>,
}
impl<T: std::fmt::Debug> std::fmt::Debug for Element<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}<->{}", self.index, self.partner_index)).unwrap();
        Ok(())
    }
}
impl<T: std::fmt::Debug> Element<T> {
    fn new(value: T) -> Self {
        Element {
            index: usize::MAX,
            partner_index: usize::MAX,
            preferences: vec![],
            value,
        }
    }

    fn has_partner(&self) -> bool {
        self.partner_index != usize::MAX
    }

    fn set_preferences(&mut self, preferences: Vec<usize>) {
        self.preferences = preferences;
    }

    fn get_preference_value(&self, e_index: usize) -> Option<usize> {
        for i in 0..self.preferences.len() {
            if self.preferences[i] == e_index {
                return Some(i);
            }
        }
        return None;
    }
    fn prefers(&self, e_index: usize) -> bool {
        self.get_preference_value(e_index) < self.get_preference_value(self.partner_index)
    }
}
#[derive(Debug, Clone)]
struct Set<T> {
    elements: Vec<T>,
}
impl<T: std::fmt::Debug> Set<Element<T>> {
    fn new(elements: Vec<Element<T>>) -> Self {
        Set { elements }
    }
    fn any_unpaired(&mut self) -> usize {
        for  e in &mut self.elements {
            if !e.has_partner() {
                return e.index;
            }
        }
        usize::MAX
    }
    fn set_preferences(&mut self, other: &mut Set<Element<T>>, randomize: bool) {
        let size = self.elements.len();
        let preferences: Vec<usize> = (0..size).collect();
        let thread_rng: &mut ThreadRng = &mut thread_rng();
        for i in 0..size {
            let self_preferences = preferences.clone();
            let other_preferences = preferences.clone();
            self.elements[i].set_preferences(self_preferences);
            self.elements[i].index = i;
            other.elements[i].set_preferences(other_preferences);
            other.elements[i].index = i;
            if randomize {
                self.elements[i].preferences.shuffle(thread_rng);
                other.elements[i].preferences.shuffle(thread_rng);
            }
        }
    }
    fn divorce(&mut self, i: usize, other_set: &mut Set<Element<T>>){
        if i < usize::MAX && self.elements[i].has_partner() {
            other_set.elements[self.elements[i].partner_index].partner_index = usize::MAX;
            self.elements[i].partner_index = usize::MAX;
        }
    }
    fn try_pair(&mut self, i: usize, j: usize, other_set: &mut Set<Element<T>>) -> bool {
        self.divorce(i, other_set);
        other_set.divorce(j, self);
        (self.elements[i].partner_index, other_set.elements[j].partner_index) = (other_set.elements[j].index, self.elements[i].index);
        return true;
    }
    
    fn stable_match_with(&mut self, other: &mut Set<Element<T>>) {
        let size = self.elements.len();
        loop {
            let i = self.any_unpaired();
            if i == usize::MAX {
                break
            }
            for j in 0..size {
                let pref_partner = &mut other.elements[self.elements[i].preferences[j]];
                if !pref_partner.has_partner() {
                    self.try_pair(i, j, other);
                    break;
                }
                else {
                    if pref_partner.prefers(i) {
                        self.try_pair(i, j, other);
                    }
                }
            }
        }
    }
}

fn generate_test_sets(n: usize) -> (Set<Element<usize>>, Set<Element<usize>>) {
    let mut a: Set<Element<usize>> = Set::new(vec![]);
    let mut b: Set<Element<usize>> = Set::new(vec![]);
    for i in 0..n {
        a.elements.push(Element::new(i));
        b.elements.push(Element::new(i));
    }
    a.set_preferences(&mut b, true);
    b.set_preferences(&mut a, true);
    (a, b)
}

fn test() {
    let men = vec![Element::new("A"),Element::new("B"),Element::new("C"),Element::new("D"),Element::new("E")];
    let women = vec!["L","M","N","O","P"];
    let mut a: Set<Element<&str>> = Set::new(men, )
}

fn main() {
    let (mut a, mut b) = generate_test_sets(3);
    a.stable_match_with(&mut b);

    println!("Men: {:?}\nWomen: {:?}", a, b);
}
