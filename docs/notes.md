This is a scratchpad for my ideas.
Do not hold me to things I put here;
everything is subject to change.

---

Element ideas:
- Behavioral: Adds a behavior to its single child element.
    - `click`: Has `action` attribute.
    - `hover`: Has `action` attribute.
    - `link`: Has a `url` attribute. Opens url when clicked.
- Content: Directly draws to the screen. Cannot have child elements.
    - `text`: Any and all text; might have `h1`, `h2`, `p` etc as presets. Cannot have children. Can have fonts, size, color.
    - `img`: Has `path` attribute.
- Layout: Draws its children. Can have any number of children.
    - `flex`: Uses flexbox algorithm. Has primary and secondary axes. Has `row` and `col` presets.

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
