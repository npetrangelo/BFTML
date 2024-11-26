# Blazingly Fast Text Markup Language
BFTML is a set of standards and reference implementation for web apps designed with the current and future needs of the web app ecosystem in mind.

The standard, so far, includes a consistent tag syntax as follows:

```
<name presets attributes>
    {children}
</name>
```
1. The `name` specifies which tag it is.
2. This is followed by the `presets`, listed in sequence.
3. Next comes the `attributes`, each with a key and value. Attributes can have the following value types:
    - Strings
    - Integers
    - Floats
    - Booleans
    - Ranges
4. After the opening tag, `children` can be any number of tags.
5. Last is the closing tag, which must repeat the name of the tag.

Optionally, tags may self close with `<tag />`.

As an example, BFTML syntax may look like so:
```xml
<button default foo="bar" answer=42 based=true pi=3.14 range=0..1>
    <foo />
</button>
```

You can verify that this syntax is supported by cloning the repo
and calling `cargo test` in your command line.

## Approximate Future Roadmap

Next, I plan to add support for various concrete tags.
I am still working out which tags I want to have in BFTML,
but when I add them, they will be type safe in that tags will only permit certain presets and attributes, and may also have stipulations on the number of children they may have.

Tags that do not adhere to these type rules will log an error,
and where possible, will render as such as well.

As BFTML will not support implicit default stylings,
all relevant styling must be explicitly specified in the tag.
However, presets may be used to specify bundles of styling, similar to TailwindCSS. There may be presets called `default` for some of the tags that will specify default styling,
but this must be indicated explicitly like any other preset.

Lastly, BFTML plans to adopt web assembly as the
backbone of its scripting runtime.
WASM is much desired by developers today, and is also
well supported in the Rust ecosystem, with several runtimes
available to choose from.

BFTML's equivalent of `script` tags will differ from HTML
in that they will only provide access to their inner subtree,
instead of the entire document. This will make them behave
similar to the component frameworks developers are now used to,
with encapsulated markup and behavior.

You may provide initial child elements to these component elements, which will be rendered even if the WASM fails to load.
Thus, BFTML will natively support graceful failure.