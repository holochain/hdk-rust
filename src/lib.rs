extern crate serde_json;

type Hash = String;

pub fn commit<S: Into<String>>(entry_type: S, entry: serde_json::Value) -> String {
    "".to_string()
}

pub fn link<S: Into<String>>(base: Hash, tag: S, target: Hash) {

}

pub fn get(entry_hash: Hash) -> String {
    "".to_string()
}

pub fn get_links<S: Into<String>>(bash: Hash, tag: S) -> Vec<Hash> {
    vec![]
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
