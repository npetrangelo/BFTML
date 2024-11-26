This is a scratchpad for my ideas.
Do not hold me to things I put here;
everything is subject to change.

---

Element ideas:
- Behavioral: Can only have one child
    - `clickable`: Has the `onclick` attribute.
    - `link`: Has a `url` attribute. Opens url when clicked.
- Content: Cannot have children
    - `text`: Any and all text; might have `h1`, `h2`, `p` etc as presets. Cannot have children. Can have fonts, size, color.
    - `img`: Has `path` attribute.
- Layout: Can have any number of children
    - `flex`: Uses flexbox algorithm. Has primary and secondary axes. Has `row` and `col` presets.

Reading this book for inspiration on which elements to support.
https://atomicdesign.bradfrost.com/

At a cursory examination, I suspect that
atoms should be supported as primitive elements,
molecules should be provided in component libraries,
some organisms may be supported in component libraries,
and the rest is application specific.
