use http::Method;
use http::Request;
use server::Server;

mod http;
mod server;
fn main() {
    let server = Server::new("localhost:8080".to_string());

    server.run();
}

// by defualt everything inside a module is private incl other modules.
// mod http {

//     pub mod request {
//         use super::method::Method;
//         pub struct Request {
//             path: String,
//             query_string: Option<String>,
//             method: Method,
//             //method: super::method::Method,
//         }
//     }

//     pub mod method {
//         pub enum Method {
//             GET,
//             POST,
//             PUT,
//             DELETE,
//             HEAD,
//             CONNECT,
//             OPTIONS,
//             TRACE,
//             PATCH,
//         }
//     }
// }
