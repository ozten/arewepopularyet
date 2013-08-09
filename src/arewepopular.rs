
extern mod extra;
extern mod std;

extern mod search;
extern mod storage;
extern mod today;

use std::hashmap::HashMap;

use extra::timer::sleep;
use extra::uv;

use search::search;
use storage::*;
use today::*;

fn analize(search_type:~str, term:&str, counts:@mut HashMap<~str, float>) -> (~str, ~[~str]) {
    let (count, repos, next_link) = search(term);
    debug!("putting %? = %?", search_type, count);
    counts.insert(search_type, count);
    println(next_link);
    (next_link, repos)
}

fn main() {
    // We can make up to 20 requests per minute.
    // Sleeping for 3100 Should work...

    let counts:@mut HashMap<~str, float> = @mut HashMap::new();
    let web_repos:~[~str] = ~[];

    let idp_repos:~[~str] = ~[];
/*
    analize(~"baseline",
            "function", counts);
    sleep(&uv::global_loop::get(), 3100);
*/

    let (web_next_link, web_repos) =
        analize(~"websites",
                "navigator.id.get OR navigator.id.request", counts);
    sleep(&uv::global_loop::get(), 3100);

    debug!("next link is %s and we've already got %?", web_next_link, web_repos);

/*
    let (idp_next_link, idp_repos) =
        analize(~"idproviders",
                "navigator.id.beginProvisioning or navigator.id.genKeyPair", counts);
    sleep(&uv::global_loop::get(), 3100);

    analize(~"facebook",
            "//connect.facebook.net/en_US/all.js", counts);

    storage::update(today(), counts);
    */
}