# Design Principles

The following are some of the most important design principles for BFTML:

### The Correct Abstraction
Abstractions are important to get right. If your abstraction is too low level, the ecosystem will develop redundancies to fill the gap. Too high, and the ecosystem becomes constrained by the limits of the platform unless and until the abstraction expands to accomodate. Although subtle, this is probably the most important of the principles.

Over the years and decades, it has become apparent that the html browser is both too high and too low simultaneously. This is because the document abstraction and the component abstraction are orthogonal: A document is a static hierarchy of content with implicit layout and styling, whereas a component is a reusable, stateful, self-contained unit of behavior. These are different things organized along different axes, and no amount of expansion of one abstraction naturally produces the other. The browser expanded upward into component territory by adding Javascript, the DOM API, and dynamic elements, and also expanded downward by exposing CSS primitives and canvas — but the document metaphor remained the foundation throughout, which is why the expansion was always awkward and never complete.

BFTML starts from the component abstraction, which is the right foundation for web apps from the beginning rather than a destination the platform has been awkwardly expanding toward for decades.

### Single Responsibility Principle
In the component paradigm, concerns are separated by component. This implies that the most primitve elements should have the most narrow concerns, which points to the single responsibility principle. As such, the primitives in BFTML aim to be as simple as possible. For example, the `border` primitive will only add a border around something. There will not be any additional behaviors for it, that will be all it does. For other behaviors, reach for other primitives.

In addition to being easier to compose with, such primitives are also easier to write and maintain because their surface area is smaller. If borders are not working right, we will know precisely what to audit.

(Contrast this with HTML, where every element has the responsibilities to support custom styling through CSS and custom behavior through Javascript. These responsibilities are rather large, and are among the main reasons html browsers are costly to maintain.)

### Consistent syntax
HTML's grammar is larger than it needs to be, primarily because some tags self close while others don't, and the behavior implicitly depends on the tag. This is cumbersome to both the developer and browser because they each must know which is which.

BFTML fixes that by forcing a consistent syntax: Self closing requires the correct syntax *for all tags*. This reduces the cognitive load for the developer who will fall into the pit of success by listening to the errors. Additionally, by making tag syntax invariant across tags, all tags can be parsed into a generic tree structure before determining which tags they are, which reduces the complexity of the parser. This two stage parsing strategy also cleanly yields two types of errors; syntax errors and tag errors.

### Explicitness
HTML and CSS have a lot of implicit default behaviors. They are often sensible, but sometimes they are not, and they can be the source of obscure bugs that are most difficult to debug because none of your code explicitly describes the undesired behavior. Hours may be spent searching the code in vain before eventually discovering the behavior in documentation instead.

BFTML will not have this problem. All bevahiors will be explicit in some way, such that you can always easily locate and audit the code responsible for your bugs. There may still be some default behaviors for convenience, but their use must be explicit.

The way this informs the language design is that by default, every parameter on an element must be explicitly specified. (The single responsibility principle makes this more manageable, because fewer responsibilities on an element means fewer parameters.) However, since that can be cumbersome, elements may also have traits carrying parameter specifications. Such traits may be used to provide default behavior, but must still be explicitly invoked to do so. Therefore, even if you use a `default` trait, you will still be able to check that trait's definition to see if a bug is there.
