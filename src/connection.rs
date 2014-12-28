use hyper::Url;
use hyper::HttpResult;
use hyper::client::Response;
use hyper::client::Request;
use hyper::HttpError;
use hyper::net::HttpConnector;

pub struct Connection<'a> {
    url: &'a str,
    pub version: String
}

impl<'a> Connection<'a> {
    pub fn new() -> Connection<'a> {
        Connection {
            url: "",
            version: String::new(),
        }
    }

    pub fn url(&'a mut self, url: &'a str) -> &'a mut Connection {
        self.url = url;
        let mut res = self.get("version").unwrap();
        let content = res.read_to_string().unwrap();
        self.version.push_str(content.as_slice());
        self
    }

    pub fn get(&'a self, path: &str) -> HttpResult<Response> {
        let url = Url::parse(format!("{}/{}", self.url, path).as_slice()).unwrap();
        Request::get(url).unwrap().start().unwrap().send()
    }
}
