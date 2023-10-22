mod element;
mod set;
use element::Element;
use set::Set;

fn test_1() {
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
    men_set.gale_shapley(&mut women_set);

    men_set.print_pairs(&women_set);

    //men_set.pair(0, 0, &mut women_set);
}

fn test_2() {
    let men = vec![
        Element::new("a"),
        Element::new("b"),
        Element::new("c"),
        Element::new("d"),
    ];
    let women = vec![
        Element::new("A"),
        Element::new("B"),
        Element::new("C"),
        Element::new("D"),
    ];
    let mut men_set: Set<Element<&str>> = Set::new(men);
    let mut women_set: Set<Element<&str>> = Set::new(women);
    men_set.set_preferences(vec![
        vec![0,1,2,3],
        vec![2,3,0,1],
        vec![2,1,3,0],
        vec![3,1,2,0],
    ]);
    women_set.set_preferences(vec![
        vec![2,3,1,0],
        vec![2,0,3,1],
        vec![1,2,3,0],
        vec![2,1,0,3],
    ]);
    men_set.gale_shapley(&mut women_set);
    men_set.print_pairs(&women_set);

    //men_set.pair(0, 0, &mut women_set);
}

fn main() {
    test_1();
}
