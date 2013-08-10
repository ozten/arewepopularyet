#[ crate_type = "lib" ];

extern mod std;

extern mod extra;


use std::hashmap::HashMap;
use std::hashmap::HashSet;
use std::io::buffered_file_writer;
use std::io::file_reader;
use std::path::Path;

use extra::json;

pub fn update_notable(today:~str, counts:@mut HashMap<~str, ~HashMap<~str, ~[~str]>>) {
    let daily_counts_path = &Path("www/data/daily_changes.json");
    // TODO copy in update style changes
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