#!/bin/bash

cd /home/ozten/Projects/arewepopularyet/
git pull origin master
for p in 'daily_counts.json'; do
  curl "http://www.arewepopularyet.com/data/${p}" > "./www/data/${p}";
done

DATE=date +"%Y-%m-%d"
git commit -m "Updated data for ${DATE}"  www/data
git push origin master
