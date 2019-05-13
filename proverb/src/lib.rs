extern crate itertools;

use itertools::Itertools;
pub fn build_proverb(list: Vec<&str>) -> String {
    list.iter()
        .tuple_windows()
        .map(|(w1, w2)| format!("For want of a {} the {} was lost.\n", w1, w2))
        .chain(
            list.first()
                .map(|w| format!("And all for the want of a {}.", w))
                .into_iter(),
        )
        .collect()
}
