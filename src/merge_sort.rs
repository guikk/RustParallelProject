use itertools::Itertools;
use diam::join;

/// pre-condition: we need an even number of levels
/// and not more than log(n) levels
fn inner_merge_sort<T: Copy + Ord + Send>(slices: (&mut [T], &mut [T]), levels: usize) {
    // START_REPLACING
    if levels == 0 {
        slices.0.sort();
        return;
    }
    let mid = slices.0.len() / 2;
    let (input, output) = slices;
    let (left_input, right_input) = input.split_at_mut(mid);
    let (left_output, right_output) = output.split_at_mut(mid);
    join(
        || inner_merge_sort((left_output, left_input), levels - 1),
        || inner_merge_sort((right_output, right_input), levels - 1),
    );
    input
        .iter_mut()
        .zip(left_output.iter().merge(right_output.iter()))
        .for_each(|(o, i)| *o = *i);
    // END_COMMENTING
}

pub fn merge_sort<T: Copy + Ord + Send + Default>(input: &mut [T]) {
    let levels = 8;
    let mut buffer: Vec<T> = std::iter::repeat_with(Default::default)
        .take(input.len())
        .collect();
    inner_merge_sort((input, &mut buffer), levels)
}

