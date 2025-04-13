pub fn insertion_sort<T: PartialOrd + Copy>(array: &mut [T]) {
    for i in 1..array.len() {
        let curr = array[i];
        let mut j = i;

        while j > 0 && array[j - 1] > curr {
            array[j] = array[j - 1];
            j -= 1;
        }
        array[j] = curr;
    }
}

fn merge<T: PartialOrd + Copy>(
    array: &mut [T],
    beginning_index: usize,
    midpoint_index: usize,
    endpoint_index: usize,
) {
    let length_left = midpoint_index - beginning_index + 1;
    let length_right = endpoint_index - midpoint_index;
    let mut left_array: Vec<T> = Vec::with_capacity(length_left);
    let mut right_array: Vec<T> = Vec::with_capacity(length_right);
    let (mut i, mut j): (usize, usize);
    for i in 0..length_left {
        left_array[i] = array[beginning_index + i];
    }
    for j in 0..length_right {
        right_array[j] = array[midpoint_index + j];
    }
    i = 0;
    j = 0;
    let mut k = beginning_index;

    while i < length_left && j < length_right {
        if left_array[i] <= right_array[j] {
            array[k] = left_array[j];
            i += 1;
        } else {
            array[k] = right_array[j];
            j += 1;
        }
        k += 1;
    }
    while i < length_left {
        array[k] = left_array[i];
        i += 1;
        k += 1;
    }
    while j < length_right {
        array[k] = right_array[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod test_merge {
    use super::*;

    #[test]
    fn merge_int_easy() {
        let mut A = [2, 4, 6, 7, 1, 2, 3, 5];
        merge(&mut A, 0, 3, 7);
        assert_eq!(A, [1, 2, 2, 3, 4, 5, 6, 7]);
    }
}

#[cfg(test)]
mod test_insertion_sort {
    use super::*;

    #[test]
    fn insertion_sort_int_easy_sorting() {
        let mut array = [10, 4, 1, 2, 3];
        insertion_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 10]);
    }
    #[test]
    fn insertion_sort_int_already_sorted() {
        let mut array = [1, 2, 3, 4];
        insertion_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4]);
    }

    #[test]
    fn insertion_sort_int_empty_array() {
        let mut array = [] as [&u64; 0];
        insertion_sort(&mut array);
        assert_eq!(array, [] as [&u64; 0]);
    }

    #[test]
    fn insertion_sort_int_inverted() {
        let mut array = [4, 3, 2, 1];
        insertion_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4]);
    }

    #[test]
    fn insertion_sort_chars_easy_sorting() {
        let mut array = ['b', 'd', 'a', 'z'];
        insertion_sort(&mut array);
        assert_eq!(array, ['a', 'b', 'd', 'z']);
    }

    #[test]
    fn insertion_sort_chars_reversed_sorting() {
        let mut array = ['z', 'd', 'b', 'a'];
        insertion_sort(&mut array);
        assert_eq!(array, ['a', 'b', 'd', 'z']);
    }
}
