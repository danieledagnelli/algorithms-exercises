pub fn insertion_sort(elements: &[u64]) {
    let n = elements.len();
    for e in elements  {
        println!("element: {}", e);
    } 

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_sort_ttt() {
        let array = [1,2,3];
        insertion_sort(&array);
        println!("Sorted array: {:?}", array);
    }
}
