use openssl::sha::Sha1;

/**
    Applies SHA1 to provided vector of a generic type.
    Returns ASCII form of hashed value
 */
pub fn to_sha1(data: &String) -> String {
    let mut hasher = Sha1::new();

    hasher.update(data.as_bytes());
    let res = hasher.finish();
    hex::encode(res)
}

#[cfg(test)]
mod tests {
    use crate::hash::to_sha1;

    #[test]
    fn test_to_sha1() {
        let data = String::from("Hello World");
        let hashed = to_sha1(&data);

        assert_eq!(hashed, "0a4d55a8d778e5022fab701977c5d840bbc486d0");
    }
}