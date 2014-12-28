
struct Keys {
    options: HashMap<&str, &str>,
    connection: &Connection
}

impl Keys {
    pub fn set() {}
    pub fn get() {}
    pub fn delete() {}
    pub fn create_dir() {}
    pub fn wait() {}
    pub fn option(&self, k: &str, v: &str) {
        self.options.insert(k, v);
    }

    fn build_url(&self, key: &str) -> Url {
        let mut path = "v2/keys/".to_string().push_str(key);
        if !self.options.is_empty() {
            path.push_str("?");
        }
        for (k, v) in self.options.drain().take(1) {
            path.push_str(k);
            path.push_str("=");
            path.push_str(v);
        }
    }
}
