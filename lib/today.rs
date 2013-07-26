#[ crate_type = "lib" ];

extern mod extra;

use extra::time::{strftime, now};

fn today() -> ~str {
    strftime(&"%Y-%m-%d", &now())
}