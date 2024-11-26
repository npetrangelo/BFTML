# Motivations

The  purpose of this project is to make a markup language
designed and optimized for making web apps.

### Why do we need this?
The existing standards are built off the document object model.
The metaphors are all extensions of the printed page.
Indeed, most websites in the early internet resembled print.
Some of them still do.

Today however, this is not true of the most popular websites.
YouTube, Facebook, Bluesky, Reddit, Discord, et. al. do not look
anything like print.

To make sites like these work, a considerable amount of additional
technology was needed on top of the document object model.

A plurality of html elements were added to support more dynamic content.
CSS was massively expanded upon to add styling options for that dynamacism.
Javascript features were added to facilitate dynamic content.

But wait, there's more!

Entire JS libraries were written to facilitate the
component model of development. Styling frameworks were created
to optimize inline styling when writing components.
Markup, styling, and behavior were all defined together,
flying in the face of the separation of concerns justifying the
original delineation between html, css, and javascript.

Clearly, something strange was happening, unforeseen by the original
architects of the web standards.

![This is madness!](https://media.tenor.com/sl9LOXQe9OMAAAAM/this-is-madness-300.gif)

### Restoring Order

All of these developments occured as the web app emerged.

Web apps wanted those extra elements.
Web apps wanted those extra styling options.
Web apps wanted that extra dynamacism.

And most of all, web apps really could not care less about
the separation of markup, styling, and behavior.

Documents, however, still do. 

This is why to this day, web app developers and document developers
continue to bicker over the merits of the standards and
the best practices of writing good web code,
seemingly unaware that they are not making the same things.

Web apps have overstayed their welcome in the house of documents.
Web apps deserve their own dedicated standards.
It's time for web apps to break free.
