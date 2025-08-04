# Intuitions
Over the years of building upon the foundations of html, css, and js, their limitations have become clear.
BFTML is a modern set of web standards, and reference implementation, that seeks to rectify these limitations.

Below are some of the limitations of each, and how BFTML rectifies them.

## HTML
### Limitations
- Inconsistent tag syntax. Some tags self close without explicit indication.
- Implicit default layout and styling that makes it harder to reason about what your markup is doing.
- Loose types. Many attributes can be applied to all tags.
- Silent failure. Invalid code is still rendered to produce dysfunctional results.
### Rectifications
- Consistent tag syntax. Self closing tags will require the self closing syntax.
- Explicitness by default. If you wish to add default styling and layout, you must specify that explicitly.
- Strict types. Attributes will be restricted to appropriate elements. For example, onclick will only be an attribute on buttons.
- Loud failure. Invalid code is not rendered, and error regions are rendered in its place.
  - Optionally, graceful failure. In case of error, a user provided fallback is rendered instead.

## CSS
### Limitations
- Implicit defaults. As with html, this makes it harder to reason about styling.
- Silent failure. Invalid code does not render at all and is difficult to find.
### Rectifications
- Explicitness by default. All styling must be expressed explicitly. Attribute bundles will help achieve this less verbosely.
- Loud failure. Invalid code is not rendered, and error regions are rendered in its place.

## Javascript
### Limitations
Now, this is the most interesting of the three facets of web standards. The main problem with it is mostly that everyone is forced to use it.
Some people might be able to live with the [billion dollar mistake](https://maximilianocontieri.com/null-the-billion-dollar-mistake) of null,
some people can abide by dynamic typing, many people don't mind the garbage collector, and being strictly single threaded makes basic things simpler.

However, there are also a lot of developers that very much don't want to code with these language characteristics.
So much so, that they would rather use a framework in Rust that compiles to web assembly which must still
generate bindings under the hood to manipulate the DOM with JS.

### Rectifications
The solution seems rather obvious: Give those people what they want, and what they want is a web assembly runtime that can directly manipulate the markup.
And so, that is what they shall have.

But we will go further. Traditional JS can manipulate the entire DOM no matter where you put it.
In BFTML, component elements will only be able to manipulate their subtree, which may contain markup to render first before the web assembly modules are loaded.
Thus, BFTML provides the safety and structure of a component framework, but leaves it up to the developer community to author higher level tools in whatever language they like.
