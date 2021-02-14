pub fn find_index<T>(v: &Vec<T>, n: &T) -> isize
where
    T: PartialOrd,
{
    let mut result: isize = -1;

    for (index, elem) in v.iter().enumerate() {
        if elem == n {
            result = index as isize;
            break;
        }
    }

    result
}
