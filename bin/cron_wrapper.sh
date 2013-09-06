#!/bin/bash
cd /Users/shout/Projects/arewepopular
cp ../arewepopular-gh-pages/data/*.json www/data/
RUST_LOG=arewepopular=4,search=4,storage=4,link_header=4 ./bin/arewepopular
cp www/data/*.json ../arewepopular-gh-pages/data
cd ../arewepopular-gh-pages/data
for f in *.json; do /Users/shout/Projects/arewepopular/bin/sort_json.js  $f; done
git commit -m"Data for `/bin/date +"%Y-%m-%d"`" d*.json w*.json i*.json
git push origin gh-pages