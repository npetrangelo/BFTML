# Blazingly Fast Text Markup Language
BFTML is a set of standards and reference implementation for web apps designed with the current and future needs of the web ecosystem in mind.

The standard, so far, includes a consistent tag syntax as follows:

```
<name presets attributes>
    {children}
</name>
```
1. The `name` specifies which tag it is.
2. This is followed by the `presets`, listed in sequence.
3. Next comes the `attributes`, each with a key and value.
4. After the opening tag, `children` can be any number of tags.
5. Last is the closing tag, which must repeat the name of the tag.

Optionally, tags may self close with `<tag />`.

You can verify that this syntax is supported by cloning the repo
and calling `cargo test` in your command line.

Next, I plan to add support for various concrete tags.
I am still working out which tags I want to have in my standard,
but when I add them, they will be type safe in that tags will only permit certain presets and attributes, and may also have stipulations on the number of children they may have.

Tags that do not adhere to these type rules will log an error,
and where possible, will render as such as well.