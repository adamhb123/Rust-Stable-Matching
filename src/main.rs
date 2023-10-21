use std::iter::zip;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
struct Element<T: Clone> {
    index: usize,
    partner_index: usize,
    value: T,
    /// Preferences here is a Vec<usize> containing the indices of the preferences of the element, from most preferred to least preferreds
    preferences: Vec<usize>
}
impl<T: Clone + std::fmt::Debug> std::fmt::Debug for Element<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?}[{:?} {:?}]<->{:?}",
            self.value, self.index, self.preferences, self.partner_index
        ))
        .unwrap();
        Ok(())
    }
}
impl<T: Clone + std::fmt::Debug> Element<T> {
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
impl<T: Clone + std::fmt::Debug> Clone for Element<T> {
    fn clone(&self) -> Self {
        let mut e = Element::new(self.value.clone());
        e.index = self.index.clone();
        e.preferences = self.preferences.clone();
        e.partner_index = self.partner_index.clone();
        e
    }
}
#[derive(Debug, Clone)]
struct Set<T> {
    elements: Vec<T>,
}
impl<T> std::ops::Index<usize> for Set<T> {
    type Output = T;
    fn index<'a>(&'a self, i: usize) -> &'a T {
        &self.elements[i]
    }
}
impl<T> std::ops::IndexMut<usize> for Set<T> {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut T {
        &mut self.elements[i]
    }
}
impl<T: Clone + std::fmt::Debug> Set<Element<T>> {
    fn new(elements: Vec<Element<T>>) -> Self {
        let mut this = Set { elements };
        // Initialize indices
        for i in 0..this.elements.len() {
            this.elements[i].index = i;
        }
        this
    }
    fn get_first_unfulfilled(&mut self) -> usize {
        for e in &mut self.elements {
            if !e.has_partner() || // TODO: Need to create proposal structure to track proposals. If a man has not propossed to all women then he is unfulfilled {
                return e.index;
            }
        }
        usize::MAX
    }
    fn get_partner_of<'a>(
        &'a self,
        i: usize,
        other: &'a mut Set<Element<T>>,
    ) -> Option<&'a mut Element<T>> {
        let partner_index = self[i].partner_index;
        match partner_index < usize::MAX {
            true => Some(&mut other[partner_index]),
            _ => None,
        }
    }
    fn set_preferences(&mut self, preferences: Vec<Vec<usize>>) {
        for i in 0..self.elements.len() {
            self.elements[i].set_preferences(preferences[i].clone());
        }
    }
    fn init_preferences(&mut self, other: &mut Set<Element<T>>, randomize: bool) {
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
    fn divorce(&mut self, i: usize, other_set: &mut Set<Element<T>>) {
        if i < usize::MAX && self.elements[i].has_partner() {
            let partner = self.get_partner_of(i, other_set);
            // Eternal sunshine for the spotless elements
            partner.unwrap().partner_index = usize::MAX; // Erase partner's partner pointer
            self[i].partner_index = usize::MAX; // Erase self element's partner pointer
        }
    }
    /// Pair self.element\[i\] with other_set.element\[j\]
    /// Where i and j are indices of their own sets respectively
    /// i.e. i is the A element's index within self, j is the B element's index within other_set
    fn pair(&mut self, i: usize, j: usize, other_set: &mut Set<Element<T>>) {
        println!("Pairing {:?} and {:?}", self[i], other_set[j]);
        self.divorce(i, other_set); // Divorce self element (i) from partner
        other_set.divorce(j, self); // Divorce new partner from their partner
                                    // First comes love, then comes:
        (
            self.elements[i].partner_index,
            other_set.elements[j].partner_index,
        ) = (other_set.elements[j].index, self.elements[i].index);
    }

    fn print_pairs(&self, other: &Set<Element<T>>) {
        let size = self.elements.len();
        let mut other_sorted = other.elements.clone();
        other_sorted.sort_by(|a,b| a.partner_index.cmp(&b.partner_index));
        let zipped: Vec<String> = zip(&self.elements, &other_sorted).map(|(a,b)| format!("({:?}, {:?})", a.value, b.value)).collect();
        println!("{}", zipped.join(","));
    }

    fn stable_match_with(&mut self, other: &mut Set<Element<T>>) {
        let size = self.elements.len();
        loop {
            let i = self.get_first_unfulfilled();
            if i == usize::MAX {
                // All paired
                break;
            }
            for j in 0..size {
                // Iterate through unpaired's preferences, try and marry them from highest to lowest
                let pref_partner = &mut other[self[i].preferences[j]];
                if !pref_partner.has_partner() {
                    println!("No partner!");
                    // preference has no partner? then pair
                    self.pair(i, pref_partner.index, other);
                    break;
                } else {
                    // preference has partner
                    if pref_partner.prefers(i) && self[i].prefers(pref_partner.index){ 
                        println!("Preferred!");
                        // preference prefers self? then pair
                        self.pair(i, pref_partner.index, other);
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
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
    a.init_preferences(&mut b, true);
    b.init_preferences(&mut a, true);
    (a, b)
}

fn test() {
    let men = vec![
        Element::new("A"),
        Element::new("B"),
        Element::new("C"),
        Element::new("D"),
        Element::new("E"),
    ];
    let women = vec![
        Element::new("L"),
        Element::new("M"),
        Element::new("N"),
        Element::new("O"),
        Element::new("P"),
    ];
    let mut men_set: Set<Element<&str>> = Set::new(men);
    let mut women_set: Set<Element<&str>> = Set::new(women);
    men_set.set_preferences(vec![
        vec![3, 1, 2, 0, 4],
        vec![4, 2, 1, 0, 3],
        vec![1, 4, 0, 3, 2],
        vec![4, 1, 3, 2, 0],
        vec![3, 0, 1, 2, 4],
    ]);
    women_set.set_preferences(vec![
        vec![3, 1, 4, 2, 0],
        vec![1, 0, 3, 2, 4],
        vec![0, 2, 4, 3, 1],
        vec![3, 0, 2, 1, 4],
        vec![1, 4, 0, 2, 3],
    ]);
    men_set.stable_match_with(&mut women_set);
    men_set.print_pairs(&women_set);
    //men_set.pair(0, 0, &mut women_set);
    println!(
        "Partner: {:?}",
        men_set.get_partner_of(0, &mut women_set).unwrap()
    );
    println!("Men: {:?}\nWomen: {:?}", men_set, women_set);
}

fn main() {
    let (mut a, mut b) = generate_test_sets(3);
    //a.stable_match_with(&mut b);
    test();
    // println!("Men: {:?}\nWomen: {:?}", a, b);
}
