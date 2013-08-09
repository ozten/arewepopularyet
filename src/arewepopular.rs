
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
/*
    analize(~"baseline",
            "function", counts);
    sleep(&uv::global_loop::get(), 3100);
*/




    let (web_next_link, web_repos1) =
        analize(~"websites",
            //navigator.id.get OR navigator.id.request

            // Testing 1 page Gigantopithecus AND blacki
            // Testing 3 pages Gigantopithecus AND bigfoot

                "Gigantopithecus AND bigfoot", counts);
    debug!("next link is %s and we've already got %?", web_next_link, web_repos1);
    web_repos.push_all_move(web_repos1);
    sleep(&uv::global_loop::get(), 3100);



/*
    let (idp_next_link, idp_repos) =
        analize(~"idproviders",
                "navigator.id.beginProvisioning or navigator.id.genKeyPair", counts);
    sleep(&uv::global_loop::get(), 3100);

    analize(~"facebook",
            "//connect.facebook.net/en_US/all.js", counts);

    storage::update(today(), counts);
    */

    // Adopters and Defectors
    /*
    1) Load yesterday's repo list
*/
    let yesterday_websites = load_repositories(~"websites");

/*
    2) Iterate all of our results
    */
    let mut web_next_link2 = web_next_link;

    if (web_next_link2.len() == 0) {
        println("Only 1 page of results");
    } else {
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
        println("Finished paginating results");
        debug!(web_repos);
    }
    let mut today_results = HashSet::new();
    for web_repos.iter().advance |r| {
        today_results.insert(r.clone());
    }

    debug!("DiFFING");
    yesterday_websites.difference(&today_results, |diff| {
        debug!("DIFFERENCE yesterday_websites %?", diff);
        true
    });

    debug!("DiFFING");
    today_results.difference(&yesterday_websites, |diff| {
        debug!("DIFFERENCE today_results %?", diff);
        true
    });
/*
    3) Compare yesterday with today
    4) Capture Adopters and Defectors
*/

/*
    5) Store lists
*/
    //save_repositories(~"websites", web_repos);
}