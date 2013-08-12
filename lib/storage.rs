#[ crate_type = "lib" ];

extern mod std;

extern mod extra;


use std::hashmap::HashMap;
use std::hashmap::HashSet;
use std::io::buffered_file_writer;
use std::io::file_reader;
use std::path::Path;

use extra::json;

pub fn update_notable(today:~str, changes:@mut HashMap<~str, ~HashMap<~str, ~[~str]>>) {
    let daily_repos_path = &Path("www/data/daily_repositories.json");
    // Read in JSON file
    let mut old_repos:~HashMap<~str, json::Json> = match file_reader(daily_repos_path) {
        Ok(dc) => {
            match json::from_reader(dc) {
                Ok(jsn) => match jsn {
                    json::Object(o) => o,
                    _ => fail!("daily_repositories.json should have a top level object")
                },
                _ => fail!("Issues parsing JSON")
            }
        },
        _ => fail!("Unable to read www/data/daily_repositories.json")
    };

    println("Updating existing JSON");

    println("Getting in update");
    changes.get(&~"websites");
    println("Finished in update");

    // Update repos
    old_repos.insert(today, jsonify_repos(changes));

    // Write out JSON file
    match buffered_file_writer(daily_repos_path) {
        Ok(fwriter) => {
            json::to_pretty_writer(fwriter, &json::Object(old_repos));
        },
        _ => fail!("Unable to write to www/data/daily_repos.json")
    }
}

fn jsonify_repos(changes:@mut HashMap<~str, ~HashMap<~str, ~[~str]>>) -> json::Json {
    /*
     {
        "websites": {
            "adopters": ["foo/bar", "baz/widget"]
        },
        "idps": {
            ...
    */
    let mut wrapped:~HashMap<~str, json::Json> = ~HashMap::new();

    let mut wrapped_websites:~HashMap<~str, json::Json> = ~HashMap::new();
    debug!(changes);
    changes.get(&~"idps");
    println("Grabbing websites from changes");
    let websites = changes.get(&~"websites");
    println("OKay");
    let adoptions = copy *websites.get(&~"adopters");
    let mut wrapped_adoption_list:~[json::Json] = ~[];
    for adoptions.iter().advance() |adoption| {
        wrapped_adoption_list.push(json::String(copy *adoption));
    }
    wrapped_websites.insert(~"adopters", json::List(wrapped_adoption_list));
    let defections = copy *websites.get(&~"defectors");
    let mut wrapped_defection_list:~[json::Json] = ~[];
    for defections.iter().advance() |defection| {
        wrapped_defection_list.push(json::String(copy *defection));
    }
    wrapped_websites.insert(~"defectors", json::List(wrapped_defection_list));
    wrapped.insert(~"websites", json::Object(wrapped_websites));

    let mut wrapped_idps:~HashMap<~str, json::Json> = ~HashMap::new();
    println("Grabbing idps from changes");
    let idps = changes.get(&~"idps");
    println("OK");
    let adoptions = copy *idps.get(&~"adopters");
    let mut wrapped_adoption_list:~[json::Json] = ~[];
    for adoptions.iter().advance() |adoption| {
        wrapped_adoption_list.push(json::String(copy *adoption));
    }
    wrapped_idps.insert(~"adopters", json::List(wrapped_adoption_list));
    let defections = copy *idps.get(&~"defectors");
    let mut wrapped_defection_list:~[json::Json] = ~[];
    for defections.iter().advance() |defection| {
        wrapped_defection_list.push(json::String(copy *defection));
    }
    wrapped_idps.insert(~"defectors", json::List(wrapped_defection_list));

    wrapped.insert(~"idps", json::Object(wrapped_idps));


    json::Object(wrapped)

}

fn wrap_websites(changes:@mut HashMap<~str, ~HashMap<~str, ~[~str]>>, data_type: &~str, change_type: &~str, wrapped:~HashMap<~str, json::Json>) {

}

pub fn update(today:~str, counts:@mut HashMap<~str, float>) {
    let daily_counts_path = &Path("www/data/daily_counts.json");
    // Read in JSON file
    let mut old_counts:~HashMap<~str, json::Json> = match file_reader(daily_counts_path) {
        Ok(dc) => {
            match json::from_reader(dc) {
                Ok(jsn) => match jsn {
                    json::Object(o) => o,
                    _ => fail!("daily_counts.json should have a top level object")
                },
                _ => fail!("Issues parsing JSON")
            }
        },
        _ => fail!("Unable to read www/data/daily_counts.json")
    };

    // Update counts
    old_counts.insert(today, jsonify(counts));

    // Write out JSON file
    match buffered_file_writer(daily_counts_path) {
        Ok(fwriter) => {
            json::to_pretty_writer(fwriter, &json::Object(old_counts));
        },
        _ => fail!("Unable to write to www/data/daily_counts.json")
    }
}

fn jsonify(counts:@mut HashMap<~str, float>) -> json::Json {
    let mut wrapped:~HashMap<~str, json::Json> = ~HashMap::new();
    for counts.iter().advance() |pair| {
        let (key, value) = pair;
        debug!(key);
        wrapped.insert(copy *key, json::Number(copy *value));
    }

    json::Object(wrapped)
}
fn load_repositories(search_type:~str) -> HashSet<~str>{

    let repos_path = &Path("www/data/" + search_type + "_repositories.json");
    // Read in JSON file
    let mut repos:~HashMap<~str, json::Json> = match file_reader(repos_path) {
        Ok(r) => {
            match json::from_reader(r) {
                Ok(jsn) => match jsn {
                    json::Object(o) => o,
                    _ => fail!("_repositories.json should have a top level object")
                },
                _ => fail!("Issues parsing JSON")
            }
        },
        _ => fail!("Unable to read www/data/daily_counts.json")
    };
    match copy *repos.get(&~"repositories") {
        json::List(l) => {
            let mut repo_set = HashSet::new();
            for l.iter().advance |item| {
                match copy *item {
                    json::String(name) => {repo_set.insert(name);},
                    _ => {}
                }
            }
            repo_set
        },
        _ => HashSet::new()
    }
}

//let daily_counts_path = &Path("www/data/" + today() + "_" + search_type + "_repositories.json");
fn save_repositories(search_type:~str, repos:~[~str]) {
    let repos_path = &Path("www/data/" + search_type + "_repositories.json");
    let mut wrapped:~[json::Json] = ~[];
    for repos.iter().advance() |repo| {
        wrapped.push(json::String(repo.clone()));
    }
    let mut data:~HashMap<~str, json::Json> = ~HashMap::new();
    data.insert(~"repositories", json::List(wrapped));
    match buffered_file_writer(repos_path) {
        Ok(fwriter) => {
            json::to_pretty_writer(fwriter, &json::Object(data));
        },
        _ => fail!("Unable to write to www/data/daily_counts.json")
    }
}