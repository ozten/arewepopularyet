# Frequently Asked Questions

# How is the Adoption Factor calculated?

It is the number of new projects that have adopted a project, divided by the number of new github projects which could probably use that feature.

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

Yes. [Please improve this methodology](https://github.com/ozten/arewepopularyet/issues)