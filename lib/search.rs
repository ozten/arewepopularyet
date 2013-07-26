#[ crate_type = "lib" ];

extern mod extra;

extern mod http_client;

extern mod link_header;

use std::hashmap::HashMap;
use std::io::buffered_file_writer;
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

pub struct RepoResponse {
    rawJson: ~[~str],
    inLinkField: bool
}

fn readJson(json: json::Json) {
    match json {
        json::List(l) => {
            //println(fmt!("Got %? items", l.len()));
            //for l.consume_iter().advance |repo| {
            for l.iter().advance |repo| {
                //println(fmt!("repo=%?\n\n", repo));

            }
            //println("A list of Objects, perhaps")
        }
        _ => {
            //println("ERROR: Unrecognized JSON format")
        }
    }
}

fn search(query:&str) {
    let search_url = "http://localhost:8002/search/code?q=" +
            query.replace(" ", "%20");

    println("SEARCH running ");
    /* search/code?q=navigator.id.get%20OR%20navigator.id.request */
    let u: Url = url::from_str(search_url).get();
    println(fmt!("%?", u));
    debug!(u);

    let mut options:HashMap<~str, ~str> = HashMap::new();
    options.insert(~"User-Agent", ~"ozten");
    // Opt into preview APIs application/vnd.github.preview
    options.insert(~"Accept", ~"application/vnd.github.preview");

    let res = @mut RepoResponse{rawJson: ~[], inLinkField: false};

    let qpath:~str = query.replace(" ", "_");

    let mut request = uv_http_request(u, options);
    do request.begin |event| {
        let f = match buffered_file_writer(&Path(
                "data/" + qpath + ".json")) {
            Ok(file) => file,
            Err(_) => fail!("Unable to open /tmp/foo")
        };
        match event {
            http_client::Error(e) => {
                println(fmt!("Ouch... error %?", e));
            },
            http_client::Status(s) => match s {
                // TODO wait... how did I break how match works here
                // I should need the pattern guard.
                StatusOK if s == StatusOK => {
                    debug!(fmt!("Status %?", s));
                    // TODO I don't need to parse Json here, actually...
                    match json::from_str(res.rawJson.concat()) {
                        Ok(json) => {
                            readJson(json);
                            f.flush();
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
                StatusUnknown => {
                    fail!("No JSON of Repositiories");
                }
            },
            http_client::HeaderField(field) => {
                let hField = str::from_bytes(field.take());
                match hField {
                    ~"link" | ~"Link" => {
                        res.inLinkField = true;
                        println("We found link");
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
                    //println(*link.replace("api.github.com", "localhost:8002"));
                    // TODO add this to incoming next url

                }
            },
            http_client::Payload(p) => {
                let data = p.take();
                res.rawJson.push(str::from_bytes(data.clone()));
                f.write_line(str::from_bytes(data.clone()));
                println("wrote some payload");
            }
        }
    }


        debug!("Sleeping for a second to respect rate limiting");
        // TODO measure time passed and wait the difference...
        sleep(&uv::global_loop::get(), 1000);
}