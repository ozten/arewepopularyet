// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use extra::net::url::Url;

pub fn build_request(url: Url, headers: ~[(~str, ~str)]) -> ~str {
    fn format_header(header:&(~str, ~str)) -> ~str {
        ~"" + header.first() + ": " + header.second()
    }
    let host = copy url.host;
    let mut path = if url.path.len() > 0 { copy url.path } else { ~"/" };

    if url.query.len() > 0 {
        let kvps = do url.query.map |pair| {
            match *pair {
                (ref key, ref value) => fmt!("%s=%s", *key, *value)
            }
        };
        path.push_str("?");
        path.push_str(kvps.connect("&"));
    }

    let mut request_header = ~[
        fmt!("GET %s HTTP/1.0", path),
        fmt!("Host: %s", host)
    ];

    println(fmt!("looping over headers %?", headers));
    for headers.iter().advance |header| {
        println(fmt!("%?", header));
        request_header.push(format_header(header));
    };

    // Two empty strings create \r\n\r\n
    // Which is the END of HTTP headers
    request_header.push(~"");
    request_header.push(~"");

    return request_header.connect("\u000D\u000A");
}

#[test]
#[allow(non_implicitly_copyable_typarams)]
fn should_request_slash_when_path_is_empty() {
    use extra::net::url;
    let url = url::from_str("http://host").get();
    assert!(url.path.is_empty());
    let headers = build_request(url);
    assert!(headers.contains("GET / "));
}
