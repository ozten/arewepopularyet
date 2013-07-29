
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

fn main() {
    // We can make up to 20 requests per minute.
    // Sleeping for 3100 Should work...

    let mut counts:HashMap<~str, float> = HashMap::new();


    counts.insert(~"baseline",
        search("function"));
    sleep(&uv::global_loop::get(), 3100);

    counts.insert(~"websites",
        search("navigator.id.get OR navigator.id.request"));
    sleep(&uv::global_loop::get(), 3100);

    counts.insert(~"idproviders",
        search("navigator.id.beginProvisioning or navigator.id.genKeyPair"));

    counts.insert(~"facebook",
        search("//connect.facebook.net/en_US/all.js"));

    storage::update(today(), counts);


}