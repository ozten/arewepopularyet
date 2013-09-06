#!/usr/bin/env node

// Rust extra::json doesn't write out JSON in any
// determanistic fashion. This Node.js script fixes
// this by reading in JSON and overwriting the file
// with sorted keys.

var fs = require('fs');
var path = require('path');

if (process.argv.length != 3) {
    console.error('Usage: ' + process.argv[1] + ' path/to/some.json');
    process.exit(1);
}
var jsonFile = path.join(process.cwd(), process.argv[2]);

function buildSortedJson(jsn) {
    if (Array.isArray(jsn)) {

        jsn.sort();
        // We could sort this bad boy...
        return jsn;
    } else if (typeof jsn === 'object') {

        var sorted = {};
        var keys = Object.keys(jsn);
        keys.sort();
        keys.forEach(function(key, i) {


            sorted[key] = buildSortedJson(jsn[key]);
        });

        return sorted;
    } else if (typeof jsn === 'array') {

    }

    return jsn;
}

fs.readFile(jsonFile, 'utf8', function(err, data) {
    if (err) {
        console.error(err);
        process.exit(2);
    }
    var originalJson = JSON.parse(data);
    var newJson = JSON.stringify(
        buildSortedJson(originalJson),
        null,
        4);
    fs.writeFile(jsonFile, newJson, 'utf8');
    //console.log(newJson);
});