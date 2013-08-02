# Are We Popular
Are we getting traction? Are we a mainstream authentication option?

One way to measure this,
is to look at adoption and abandonment rates in github codebases.

## Criteria

Several searches are performed daily against github's code search API.

See [searches.json](etc/searches.json).

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
* Seperate github project #s from the Adoption Factor
* make a directory for today's run and put full search results in there
* Take rate-limiting into account and eventually get all the data
  * Follow Link and get all results
* use conf.json
* total_count should be an int, not a float
* implement Basic Auth (currently using client_id and client_secret)