This is a scratchpad for my ideas.
Do not hold me to things I put here;
everything is subject to change.

---

Primitive element ideas:
- Behavioral: Adds a behavior to its single child element.
    - `click`: Has `action` attribute.
    - `hover`: Has `action` attribute.
    - `link`: Has a `url` attribute. Opens url when clicked.
- Graphical: Draws to the screen.
    - Content: Renders content on screen. No children.
        - `text`: Any and all text; might have `h1`, `h2`, `p` etc as presets. Cannot have children. Can have fonts, size, color.
        - `img`: Has `path` attribute.
    - Decoration: Renders stuff around single child.
        - `border`: Has `top`, `left`, `bottom`, `right`, `style` attributes
        - `margin`: Has `top`, `left`, `bottom`, `right` attributes
        - `onion`: Has `opacity` attribute
    - Layout: Distributes its children on screen.
        - `flex`: Uses flexbox algorithm. Has primary and secondary axes. Has `row` and `col` presets.
        - `layers`: Places children on top of each other.

Custom primitives only supported for graphical elements

Reading this book for inspiration on which elements to support.
https://atomicdesign.bradfrost.com/

At a cursory examination, I suspect that
atoms should be supported as primitive elements,
molecules should be provided in component libraries,
some organisms may be supported in component libraries,
and the rest is application specific.

Deconstructed button:
```xml
<click f=f()>
    <box color=red>
        <text color=white>
            SUBSCRIBE
        </text>
    </box>
</click>
```
Note that the box element can be substitute with one for any shape.

Primitive elements should be compiled to WASM. You can add your own.
The WASI interface will give the elements access to gpu functions
and a few others. Requires advanced coding knowledge to write.
May take a while to compile.

Components composed of the primitives should use an interpreted language.
Interpreter and runtime should be compiled to WASM.
Builds near instantaneously for rapid HMR.
