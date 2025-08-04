This is a scratchpad for my ideas.
Do not hold me to things I put here;
everything is subject to change.

---

Primitive element ideas:
- Behavioral: Reacts to IO. Wraps other elements.
    - `onClick`
    - `onRightClick`
    - `onHold`
    - `onHover`
    - `onPress`
    - `onRelease`
    - `onScroll`
    - `onKey`
- `embed`
- Graphical: Draws to the screen.
    - Content: Renders content on screen. No children.
        - `text`: Any and all text; might have `h1`, `h2`, `p` etc as presets. Cannot have children. Can have fonts, size, color.
        - `markdown`: The format we know and love, to concisely structure large text blobs.
        - `img`: Has `path` attribute.
        - `video`
    - Wrapper: Renders stuff around single child.
        - `box`
        - `outline`: Has `top`, `left`, `bottom`, `right`, `style` attributes
        - `margin`: Has `top`, `left`, `bottom`, `right` attributes
        - `blend`: Has `opacity` and `mode` attributes
        - `shadow`
    - Layout: Distributes its children on screen.
        - `row`
        - `column`
        - `flex`: Uses flexbox algorithm. Has primary and secondary axes. Has `row` and `col` presets.
        - `grid`
        - `layers`: Places children on top of each other.
- Audio: Plays a sound when element is "visible" (e.g. when parent element is visible)
    - TTS: Speaks the provided text, maybe with the specified voice (if good crates exist for that)


Custom primitives only supported for graphical elements and audio elements

Reading this book for inspiration on which elements to support.
https://atomicdesign.bradfrost.com/

At a cursory examination, I suspect that
atoms should be supported as primitive elements,
molecules should be provided in component libraries,
some organisms may be supported in component libraries,
and the rest is application specific.

Deconstructed button:
```xml
<event onclick=f()>
    <box color=red>
        <text color=white>
            SUBSCRIBE
        </text>
    </box>
</event>
```
Note that the box element can be substitute with one for any shape.

Primitive elements should be compiled to WASM. You can add your own.
The WASI interface will give the elements access to gpu functions
and a few others. Requires advanced coding knowledge to write.
May take a while to compile.

Components composed of the primitives should use an interpreted language.
Interpreter and runtime should be compiled to WASM.
Builds near instantaneously for rapid HMR.


Next up:
- Add wasmtime and make interface for wasm shaders
- Accept shaders of all types supported by WGPU to define custom elements, including SPIR-V.
Identify which shader type to use by file extension.
