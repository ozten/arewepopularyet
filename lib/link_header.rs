// rustc link_header

#[ crate_type = "lib" ];

// Given a Link header, parse out the next url
// <https://api.github.com/repositories?since=364>; rel="next", <https://api.github.com/repositories{?since}>; rel="first"
fn parse(header: ~str) -> @~str {
    match header.find('<') {
        Some(start) => {
            match header.find('>') {
                Some(end) => {
                    let link:@~str = //@"foo";
                    @(header.slice(start + 1, end).to_owned());
                    return link;
                },
                _ => { fail!("Missing >") }
            }

        },
        _ => {fail!("Missing <") }
    }
}