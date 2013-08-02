# Frequently Asked Questions

# How is the Adoption Factor calculated?

It is the number of new github projects that have adopted a technology, divided by the number of new github projects which could probably adopt that technology.

Basically, projects that adopted Persona versus total number of projects that use JavaScript added to Github.

The number of projects, since the last time we measured, to be more precise.

So if 4 projects started showing up as having just adopted Persona, and 40 new web based projects were added to Github, then 4/40 is the Adoption Factor, which is **0.1**.

# What do these numbers mean in the Raw Data?

You can look directly at the [Raw Data](http://www.areweawesomeyet.com/data/daily-count.js). Here is the schema.

    {
      "2013-07-29": {       -- Year, Month, Day

        "websites": 1055,   -- Total # of projects using Persona's RP APIs
        "idproviders": 79,  -- Total # of projects using Persona's IdP APIs
        "facebook": 34759,  -- Total # of projects using Facebook Connect
        "baseline": 160102342 -- Total # of webby projects
    },

# How do you calculate these results?
We use github's [search APIs](https://github.com/ozten/arewepopularyet/blob/master/src/arewepopular.rs#L25)

The **baseline** is for repos who's source code uses `function` which is a pretty webby keyword.

# Isn't that flawed?

[Yes, notes here](./methodology.md). [Please improve this methodology](https://github.com/ozten/arewepopularyet/issues)