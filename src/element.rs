use std::fmt::Debug;
pub struct Element<T: Clone> {
    pub index: usize,
    pub partner_index: usize,
    pub value: T,
    /// Element.preferences is a Vec<usize> containing the indices of the preferences of the element, from most preferred to least preferred
    pub preferences: Vec<usize>,
}

impl<T: Clone + Debug> Debug for Element<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?}[{:?} {:?}]<->{:?}",
            self.value, self.index, self.preferences, self.partner_index
        ))
        .unwrap();
        Ok(())
    }
}
impl<T: Clone + Debug> Element<T> {
    pub fn new(value: T) -> Self {
        Element {
            index: usize::MAX,
            partner_index: usize::MAX,
            value,
            preferences: vec![],
        }
    }
    pub fn has_partner(&self) -> bool {
        self.partner_index != usize::MAX
    }
    pub fn set_preferences(&mut self, preferences: Vec<usize>) {
        self.preferences = preferences;
    }
    pub fn get_preference_value(&self, e_index: usize) -> usize {
        for i in 0..self.preferences.len() {
            if self.preferences[i] == e_index {
                return i;
            }
        }
        return usize::MAX;
    }
    pub fn prefers(&self, e_index: usize) -> bool {
        self.get_preference_value(e_index) < self.get_preference_value(self.partner_index)
    }
}
impl<T: Clone + Debug> Clone for Element<T> {
    fn clone(&self) -> Self {
        let mut e = Element::new(self.value.clone());
        e.index = self.index.clone();
        e.preferences = self.preferences.clone();
        e.partner_index = self.partner_index.clone();
        e
    }
}