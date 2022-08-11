pub fn vec_to_string<T: ToString>(vector: &Vec<T>) -> String {
    let mut s = String::from("");

    for v in vector {
        s.push_str(v.to_string().as_str());
        s.push_str(" ");
    }

    s
}

#[cfg(test)]
mod test {
    use crate::utils::vec_to_string;

    #[test]
    fn test_vec_to_string() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(
            vec_to_string(&v),
            "1 2 3 4 "
        );
    }
}