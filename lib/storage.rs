#[ crate_type = "lib" ];

extern mod std;

extern mod extra;


use std::hashmap::HashMap;
use std::io::buffered_file_writer;
use std::io::file_reader;
use std::path::Path;

use extra::json;

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
