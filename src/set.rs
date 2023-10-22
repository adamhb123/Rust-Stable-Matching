use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::zip;

use crate::element::Element;

type IndexMap = HashMap<usize, Vec<usize>>;

pub struct Set<T> {
    elements: Vec<T>,
    proposal_map: IndexMap
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
impl<T: Clone + Debug> Set<Element<T>> {
    pub fn new(elements: Vec<Element<T>>) -> Self {
        let mut this: Set<Element<T>> = Set {
            elements,
            proposal_map: HashMap::new(),
        };
        // Initialize indices
        for i in 0..this.elements.len() {
            this.elements[i].index = i;
            this.proposal_map.insert(i, vec![]);
        }
        this
    }
    fn get_first_unfulfilled(&mut self) -> usize {
        let size = self.elements.len();
        // Check first for unpartnered elements:
        for e in &mut self.elements {
            // If the element does not have a partner
            if !e.has_partner() {
                return e.index;
            }
        }
        // Second for elements who have yet to ask all:
        for e in &mut self.elements {
            // If the element does not have a partner
            if self.proposal_map[&e.index].len() < size {
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
    pub fn set_preferences(&mut self, preferences: Vec<Vec<usize>>) {
        if preferences.len() != self.elements.len() {
            panic!("Preferences must be equal to number of elements");
        }
        for i in 0..self.elements.len() {
            self.elements[i].set_preferences(preferences[i].clone());
        }
    }
    pub fn init_preferences(&mut self, other: &mut Set<Element<T>>, randomize: bool) {
        let size = self.elements.len();
        let preferences: Vec<usize> = (0..size).collect();
        let thread_rng: &mut ThreadRng = &mut thread_rng();
        for i in 0..size {
            self.elements[i].set_preferences(preferences.clone());
            self.elements[i].index = i;
            other.elements[i].set_preferences(preferences.clone());
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
        ) = (j, i);
    }

    pub fn reset(&mut self){
        for i in 0..self.elements.len() {
            self[i].partner_index = usize::MAX;
            self.proposal_map.get_mut(&i).unwrap().clear();
        }
    }

    pub fn print_pairs(&self, other: &Set<Element<T>>) {
        let mut other_sorted = other.elements.clone();
        other_sorted.sort_by(|a, b| a.partner_index.cmp(&b.partner_index));
        let zipped: Vec<String> = zip(&self.elements, &other_sorted)
            .map(|(a, b)| format!("({:?}, {:?})", a.value, b.value))
            .collect();
        println!("{}", zipped.join(","));
    }

    pub fn gale_shapley(&mut self, other: &mut Set<Element<T>>) {
        self.reset();
        let size = self.elements.len();
        loop {
            let i = self.get_first_unfulfilled();
            if i == usize::MAX {
                // All paired
                break;
            }
            for j in 0..size {
                let pref_partner = &other[self[i].preferences[j]];
                println!("{:?} asks {:?}", self.elements[i].value, pref_partner.value);
                if !(self[i].has_partner() || pref_partner.has_partner()) {
                    //println!("No partners!");
                    self.pair(i, pref_partner.index, other);
                    break;
                } else if !self.proposal_map[&i].contains(&pref_partner.index) {
                    /* println!(
                        "Adding {:?}.proposal_map <-- {:?} ",
                        self[i].value, pref_partner.value
                    );*/
                    self.proposal_map
                        .get_mut(&i)
                        .unwrap()
                        .push(pref_partner.index);
                    /*println!("Man {:?} prefer? Woman {:?}", i, pref_partner.index);
                    println!("Man partner current {:?}", self[i].partner_index);
                    println!(
                        "M pref W {:?} W pref M{:?}",
                        self[i].prefers(pref_partner.index),
                        pref_partner.prefers(i)
                    );*/
                    // preference has partner
                    if pref_partner.prefers(i) && self[i].prefers(pref_partner.index) {
                        println!("Preferred!");
                        // preference prefers self? then pair
                        self.pair(i, pref_partner.index, other);
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        // println!("{:?}", self.proposal_map);
    }
}

pub fn generate_test_sets(n: usize) -> (Set<Element<usize>>, Set<Element<usize>>) {
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