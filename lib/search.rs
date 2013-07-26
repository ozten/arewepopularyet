#[ crate_type = "lib" ];

extern mod extra;

// extern mod libc;

extern mod http_client;

extern mod link_header;
extern mod secrets;
extern mod today;

use std::hashmap::HashMap;
use std::io::buffered_file_writer;
use std::libc::{S_IRUSR, S_IWUSR, S_IXUSR};
use std::os::mkdir_recursive;
use std::path::Path;
use std::str;
use std::result::{Ok, Err};


use extra::json;
use extra::json::{Object, List, String, Number};
use extra::net::url::Url;
use extra::net::url;
use extra::timer::sleep;
use extra::uv;

use http_client::uv_http_request;
use secrets::qs;
use today::*;

pub struct RepoResponse {
    rawJson: ~[~str],
    inLinkField: bool,
    file: @std::io::Writer:'static,
    total_count: float
}

fn readJson(json: json::Json) -> float {
    match json {
        json::Object(o) => {
            match copy *o.get(&~"total_count") {
                Number(n) => n,
                _ => fail!("Expected top level property total_count")
            }
        }
        _ => fail!("Expected Object at top level of JSON")
    }
}

fn search(query:&str) -> float {
    // Use a Node.js proxy server, since rust-http-client can't do https
    let search_url = "http://localhost:8002/search/code?q=" +
            query.replace(" ", "%20") +
            secrets::qs();

    let u: Url = url::from_str(search_url).get();
    debug!(u);

    let mut options:HashMap<~str, ~str> = HashMap::new();
    // Github thing... use your app name
    options.insert(~"User-Agent", ~"ozten");
    // Opt into preview APIs application/vnd.github.preview
    options.insert(~"Accept", ~"application/vnd.github.preview");

    let qpath:~str = query.replace(" ", "_");

    // To hedge our bets, let's save the results to a file.
    mkdir_recursive(&Path("data/" + today()),
            (S_IRUSR | S_IWUSR | S_IXUSR) as i32);
    let f = match buffered_file_writer(&Path(
                "data/" + today() + "/" + qpath + ".json")) {
            Ok(file) => file,
            Err(_) => fail!("Unable to open " + "data/" + today() + "/" + qpath + ".json")
    };

    let res = @mut RepoResponse{
        rawJson: ~[], inLinkField: false,
        file: f, total_count: -1.0
    };

    let mut request = uv_http_request(u, options);

    do request.begin |event| {
        match event {
            http_client::Error(e) => {
                println(fmt!("Ouch... error %?", e));
                fail!("http error");
            },
            http_client::Status(s) => match s {
                StatusOK if s == StatusOK => {
                    debug!(fmt!("Status %?", s));
                    match json::from_str(res.rawJson.concat()) {
                        Ok(json) => {
                            res.file.flush();
                            res.total_count = readJson(json);
                        }
                        Err(e) => {
                            println(fmt!("Error parsing JSON %?", e));
                            fail!("Can't read JSON");
                        }
                    }
                }
                StatusFound if s == StatusFound => {
                    debug!(fmt!("UNEXPECTED: Redirected? %?", s));
                }
                StatusUnknown => fail!("No JSON of Repositiories")
            },
            http_client::HeaderField(field) => {
                let hField = str::from_bytes(field.take());
                match hField {
                    ~"link" | ~"Link" => {
                        res.inLinkField = true;
                    },
                    _ => ()
                }
            },
            http_client::HeaderValue(field) => {
                if (res.inLinkField) {
                    res.inLinkField = false;
                    let hValue = str::from_bytes(field.take());
                    println("Queing up next page from ");
                    let link:@~str = link_header::parse(hValue);
                }
            },
            http_client::Payload(p) => {
                let data = p.take();
                res.rawJson.push(str::from_bytes(data.clone()));
                res.file.write_line(str::from_bytes(data.clone()));
            }
        }
    }
    res.total_count
}