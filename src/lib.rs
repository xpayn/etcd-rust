extern crate hyper;
extern crate url;

use connection::Connection;
pub mod connection;


#[test]
fn it_works() {
    let mut etcd = Connection::new();
    etcd.url("http://localhost:2379");
    let mut v = etcd.version;
    println!("etcd {}", v);
}
