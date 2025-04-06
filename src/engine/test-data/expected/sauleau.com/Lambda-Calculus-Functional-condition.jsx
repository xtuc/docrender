import { createElement, Fragment } from "https://unpkg.com/es-react";

function Heading({ style, id, level, children }) {
  const tag = "h" + level;
  return createElement(tag, { style, id }, children);
}

function Paragraph({ children }) {
  return createElement("p", {}, children);
}

function Code({ children }) {
  return createElement("code", {}, children);
}

function Image({ src }) {
  return createElement("img", { src });
}

function Link({ href }) {
  return createElement("a", { href });
}

function List({ children }) {
  return createElement("ul", {}, children);
}

function ListItem({ children }) {
  return createElement("li", {}, children);
}

export default function Doc({
  heading = Heading,
  paragraph = Paragraph,
  code = Code,
  image = Image,
  link = Link,
  list = List,
  listItem = ListItem,
} = {}) {
  return createElement(
    Fragment,
    {},
    createElement(heading, { level: 2, id: "pb-todo" }, `Lambda Calculus`),
    createElement(
      paragraph,
      {},
      `If you have never heard about lambda calculus, here is a quick definition found on Wikipedia:`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `“Lambda calculus (also written as λ-calculus) is a formal system in mathematical logic for expressing computation based on function abstraction and application using variable binding and substitution.”`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Basically, this is the syntax:`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `&lt;function&gt; ::= λ &lt;variable-list&gt; . &lt;expression&gt;`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `An expression is also a variable.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `For example, here is the `,
      createElement(
        link,
        {
          style: {
            textDecoration: "underline",
            color: "rgb(6.666667%, 33.333336%, 80%)",
          },
          href: "https://en.wikipedia.org/wiki/Identity_function",
        },
        `identity function from mathematics`,
      ),
      `:`,
      `
`,
    ),
    createElement(paragraph, {}),
    createElement(
      code,
      { class: "language-lambda calculus" },
      `<codeblock-js>identity = λx.x
</codeblock-js>`,
    ),
    createElement(
      paragraph,
      {},
      `A more concrete example is its usage to represent types. In the following example:`,
      `
`,
    ),
    createElement(paragraph, {}),
    createElement(
      code,
      { class: "language-js" },
      `<codeblock-js>function upper(str) {
  return str.toUpperCase()
}
</codeblock-js>`,
    ),
    createElement(
      paragraph,
      {},
      `has the type:`,
      `
`,
    ),
    createElement(paragraph, {}),
    createElement(
      code,
      { class: "language-lambda calculus" },
      `<codeblock-js>t' = λ t . t

// We can reduce it to infer the return type
t' = λ string . string
t' = string

// The result type is \`string\`
</codeblock-js>`,
    ),
    createElement(paragraph, {}),
    createElement(
      heading,
      { level: 4, id: "pb-todo" },
      `Let's define booleans`,
    ),
    createElement(
      code,
      { class: "language-lambda calculus" },
      `<codeblock-js>true = λx.λy.x
false = λx.λy.y    
</codeblock-js>`,
    ),
    createElement(
      heading,
      { level: 4, id: "pb-todo" },
      `Define the conditional operator`,
    ),
    createElement(
      code,
      { class: "language-lambda calculus" },
      `<codeblock-js>if = λb.λt.λf.b t f
</codeblock-js>`,
    ),
    createElement(
      heading,
      { level: 4, id: "pb-todo" },
      `Define the logical operators`,
    ),
    createElement(
      code,
      { class: "language-lambda calculus" },
      `<codeblock-js>and = λbb'. if b b' false
or = λbb'. if b true b'
not = λb.if b false true
</codeblock-js>`,
    ),
    createElement(heading, { level: 3, id: "pb-todo" }, `Example`),
    createElement(
      paragraph,
      {},
      `I decided to use OCaml for this example because it has partial application and a nice REPL.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Since the conditional and logical operators are already in the language, I will prepend them with an underscore.`,
      `
`,
    ),
    createElement(paragraph, {}),
    createElement(
      heading,
      { level: 4, id: "pb-todo" },
      `Translate definitions into OCaml`,
    ),
    createElement(
      code,
      { class: "language-ocaml" },
      `<codeblock-js>let _true x y = x;;
(* - : 'a -> 'b -> 'a =  *)

let _false x y = y;;
(* - : 'a -> 'b -> 'b =  *)
</codeblock-js>`,
    ),
    createElement(
      paragraph,
      {},
      `Note that it's similar to how an identity function works.`,
      `
`,
    ),
    createElement(paragraph, {}),
    createElement(
      code,
      { class: "language-ocaml" },
      `<codeblock-js>let _if b t f = b t f;;
(* - : ('a -> 'b -> 'c) -> 'a -> 'b -> 'c =  *)
</codeblock-js>`,
    ),
    createElement(
      paragraph,
      {},
      `For the sake of simplicity, I won't declare all the operators for now. We can already evaluate some simple conditions.`,
      `
`,
    ),
    createElement(paragraph, {}),
    createElement(
      code,
      { class: "language-ocaml" },
      `<codeblock-js>(* This is my condition, for now it's a constant \`true\` *)
let myCond = _true;;

_if myCond 1 0;;
(* - : int = 1 *)

(* and if my condition is \`false\` *)
let myCond2 = _false;;

_if myCond2 1 0;;
(* - : int = 0 *)
</codeblock-js>`,
    ),
    createElement(heading, { level: 3, id: "pb-todo" }, `Conclusion`),
    createElement(
      paragraph,
      {},
      `To be honest, I don't know why you would use this in real life code. There is no need to redefine the built-in condition mechanism.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Thanks to `,
      createElement(
        link,
        {
          style: {
            textDecoration: "underline",
            color: "rgb(6.666667%, 33.333336%, 80%)",
          },
          href: "https://twitter.com/dwillems42",
        },
        `Danny Willems`,
      ),
      ` for introducing me to this.`,
      `
`,
    ),
    createElement(paragraph, {}),
  );
}
