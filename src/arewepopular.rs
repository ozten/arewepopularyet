
extern mod extra;
extern mod std;

extern mod search;
extern mod storage;
extern mod today;

use std::hashmap::HashMap;
use std::hashmap::HashSet;

use extra::timer::sleep;
use extra::uv;

use search::{search, get_search};
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
    let mut web_repos:~[~str] = ~[];

    let mut idp_repos:~[~str] = ~[];

    analize(~"baseline",
            "function", counts);
    sleep(&uv::global_loop::get(), 3100);

    let (web_next_link, web_repos1) =
        analize(~"websites",
            // Testing 1 page Gigantopithecus AND blacki
            // Testing 3 pages Gigantopithecus AND bigfoot
                "\"navigator.id.get\" OR \"navigator.id.request\"", counts);
    web_repos.push_all_move(web_repos1);
    sleep(&uv::global_loop::get(), 3100);

    let (idp_next_link, idp_repos1) =
        analize(~"idproviders",
                "\"navigator.id.beginProvisioning\" OR \"navigator.id.genKeyPair\"", counts);
    idp_repos.push_all_move(idp_repos1);
    sleep(&uv::global_loop::get(), 3100);

    analize(~"facebook",
            "//connect.facebook.net/en_US/all.js", counts);

    storage::update(today(), counts);

    // Adopters and Defectors

    //1) Load yesterday's repo list

    let yesterday_websites = load_repositories(~"websites");
    let yesterday_idps = load_repositories(~"idps");


    //2) Iterate all of our website results

    let mut web_next_link2 = web_next_link;
    if (web_next_link2.len() != 0) {
        loop {
            let (count, repos, next_link) = get_search(web_next_link2.replace("https://api.github.com", "http://localhost:8002"));
            if (next_link.len() == 0) {
                break;
            }
            web_next_link2 = next_link.clone();

            debug!("next link is %s and we've already got %?", web_next_link2, repos);
            web_repos.push_all_move(repos);
            sleep(&uv::global_loop::get(), 3100);
        }
    }
    let mut today_web_results = HashSet::new();
    for web_repos.iter().advance |r| {
        today_web_results.insert(r.clone());
    }

    let mut notable:@mut HashMap<~str, ~HashMap<~str, ~[~str]>> = @mut HashMap::new();
    let mut notable_websites:~HashMap<~str, ~[~str]> = ~HashMap::new();

    let mut new_websites = ~[];
    yesterday_websites.difference(&today_web_results, |diff| {
        debug!("Adopters yesterday_websites %?", diff);
        new_websites.push(diff.clone());
        true
    });
    notable_websites.insert(~"adopters", new_websites);

    let mut missing_websites = ~[];
    today_web_results.difference(&yesterday_websites, |diff| {
        debug!("Defectors today_web_results %?", diff);
        missing_websites.push(diff.clone());
        true
    });
    notable_websites.insert(~"defectors", missing_websites);

    notable.insert(~"websites", notable_websites);
    println("Getting early");
    notable.get(&~"websites");
    println("Finished Getting early");
    //2) Iterate all of our idp results
//aok
    let mut idp_next_link2 = idp_next_link;
    if (idp_next_link2.len() != 0) {
        loop {
            let (count, repos, next_link) = get_search(idp_next_link2.replace("https://api.github.com", "http://localhost:8002"));
            if (next_link.len() == 0) {
                break;
            }
            idp_next_link2 = next_link.clone();

            idp_repos.push_all_move(repos);
            sleep(&uv::global_loop::get(), 3100);
        }
    }
    let mut today_idp_results = HashSet::new();
    for idp_repos.iter().advance |r| {
        today_idp_results.insert(r.clone());
    }

    let mut notable_idps:~HashMap<~str, ~[~str]> = ~HashMap::new();

    let mut new_idps = ~[];
    yesterday_idps.difference(&today_idp_results, |diff| {
        debug!("Adopters yesterday_idps %?", diff);
        new_idps.push(diff.clone());
        true
    });
    notable_idps.insert(~"adopters", new_idps);

    let mut missing_idps = ~[];
    today_idp_results.difference(&yesterday_idps, |diff| {
        debug!("Defectors today_idp_results %?", diff);
        missing_idps.push(diff.clone());
        true
    });
    notable_idps.insert(~"defectors", missing_idps);

    notable.insert(~"idps", notable_idps);


/*
// AOK
    let mut fake_adopters = ~[~"foo", ~"bar", ~"baz"];
    notable_websites.insert(~"adopters", fake_adopters);
    let mut fake_defectors = ~[~"foo", ~"bar", ~"baz"];
    notable_websites.insert(~"defectors", fake_defectors);

    notable.insert(~"websites", notable_websites);

    let mut fake_iadopters = ~[~"idfoo", ~"idbar", ~"udbaz"];
    notable_idps.insert(~"adopters", fake_iadopters);
    let mut fake_idefectors = ~[~"idfoo", ~"idbar", ~"idbaz"];
    notable_idps.insert(~"defectors", fake_idefectors);

    notable.insert(~"idps", notable_idps);

    println("INserted IdPs");
    //END AOK
    */

    println("Getting b4");
    notable.get(&~"websites");
    println("Finished b4");

    println("INserted websites and IdPs");
    storage::update_notable(today(), notable);
    println("SAVING repositories");


/*
    5) Store lists
*/
    save_repositories(~"websites", web_repos);
    save_repositories(~"idps", idp_repos);
}