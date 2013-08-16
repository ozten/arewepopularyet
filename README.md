# Are We Popular
Are we getting traction? Are we a mainstream authentication option?

One way to measure this,
is to look at adoption and abandonment rates in github codebases.

## Criteria

Several searches are performed daily against github's code search API.

See [searches.json](etc/searches.json).

## Current behavior
1) Get first page of results for various search terms, update daily_counts.json
2) Retrieve all search results for websites and idps, record all repo names
3) Load yesterday's repos and diff against today's list
4) Update list of adopters and defectors
5) For both websites and idps, overwrite the list of known repos with today's list

## Deployment

Project is deployed on dotcloud and has the following layout...

```
code (contents of this repo)
    bin
        arewepopular
    www
        data/daily_count.js
data
    2013-07-26/api_call.json
```
www is the static website.

The arewepopular program runs once per day and updates daily_count.js,
which is also commited back to the codebase.

data is a persistent place on the server where we save out the
metadata around the search, for future use. This gets periodically
slurped down to a backup and deleted. This is outside of this codebase.

## TODOs
* Show Adopters / Abandoneers
* Robust cron jobs and redundant deployment / backup
* use conf.json
* total_count should be an int, not a float
* implement Basic Auth (currently using client_id and client_secret)
* Break .json files up by month and have UI be able to access across months
* Tighten up Rust code