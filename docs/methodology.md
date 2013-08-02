# Methodology

The data that powers this website is fuzzy at best.

## Considerations

Given unlimited resources, I'd spider the web and properly parse HTML pages for uses of Persona APIs.

I've looked into 80legs and Commons Crawl.

Thinking smaller, if one could spider all the projects in Github, bitbucket, sourceforge, etc that would be great. Using Github's `/repositories` API would take 90 days to get the metadata for all the repos. Wow.

There are "source code search engines", which is our current method of measuring usage.

* Github Search API
* [meanpath](https://meanpath.com/)

## Imperfect
The following are reasons why Source code search engines are imperfect
* Search Indexes rebuilt on an unknown schedule
* Simple text matching can give false positives
  * As well as miss some more advanced implementations of Persona

The following are known issues with our search terms:

* We miss projects that use a library that provides Persona support, such as OmniAuth or Passport