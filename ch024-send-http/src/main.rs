use reqwest::blocking::{Client, ClientBuilder};
use reqwest::redirect::Policy;

fn main() {
    let http_client = Client::new();
    let http_result = http_client.get("https://ssuchichen.com").send();
    if http_result.is_ok() {
        println!("{:#?}", http_result.ok().unwrap());
    } else {
        println!("{:#?}", http_result.err());
    }

    let post_result = http_client
        .post("https://www.rust-lang.org/")
        .body("{\"first_name\":\"Alice\"}")
        .header("Content-Type", "application/json")
        .send();
    if post_result.is_ok() {
        println!("{:#?}", post_result.ok().unwrap().text().unwrap());
    } else {
        println!("{:#?}", post_result.err());
    }

    let redir_policy = Policy::limited(10);
    let redir_client = ClientBuilder::new()
        .redirect(redir_policy)
        .build()
        .ok()
        .unwrap();

    let redir_result = redir_client.get("https://www.rust-lang.org/").send();
    if redir_result.is_ok() {
        println!("{:#?}", redir_result.ok().unwrap().text().unwrap());
    } else {
        println!("{:#?}", redir_result.err());
    }
}
