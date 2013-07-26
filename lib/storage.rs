#[ crate_type = "lib" ];

extern mod std;

extern mod extra;


use std::hashmap::HashMap;
use std::io::buffered_file_writer;
use std::io::file_reader;
use std::path::Path;

use extra::json;

pub fn update(counts:HashMap<~str, float>) {
    let daily_counts_path = &Path("data/daily_counts.json");
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
        _ => fail!("Unable to read data/daily_counts.json")
    };

    // Update counts
    old_counts.insert(~"2013-07-26", jsonify(counts));
    // Write out JSON file
    match buffered_file_writer(daily_counts_path) {
        Ok(fwriter) => {
            json::to_pretty_writer(fwriter, &json::Object(old_counts));
        },
        _ => fail!("Unable to write to data/daily_counts.json")
    }
}

fn jsonify(counts:HashMap<~str, float>) -> json::Json {
    let mut wrapped:~HashMap<~str, json::Json> = ~HashMap::new();
    // TODO make this dynamic, iterate through the keys
    wrapped.insert(~"baseline", json::Number(copy *counts.get(&~"baseline")));
    wrapped.insert(~"websites", json::Number(copy *counts.get(&~"websites")));
    wrapped.insert(~"idproviders", json::Number(copy *counts.get(&~"idproviders")));
    json::Object(wrapped)
}
