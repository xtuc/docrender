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
    createElement(heading, { level: 2, id: "pb-todo" }, `What is this?`),
    createElement(
      paragraph,
      {},
      `This refers to the proposal `,
      createElement(
        link,
        {
          style: {
            textDecoration: "underline",
            color: "rgb(6.666667%, 33.333336%, 80%)",
          },
          href: "https://github.com/tc39/proposal-pipeline-operator",
        },
        `https://github.com/tc39/proposal-pipeline-operator`,
      ),
      `.`,
      `
`,
    ),
    createElement(
      heading,
      { level: 2, id: "pb-todo" },
      `Define the compose function`,
    ),
    createElement(
      paragraph,
      {},
      `While ∘ is the official symbol in mathematics, it's not a valid JavaScript identifier.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `I will be using ᐅ which looks like the pipe operator |&gt; in most languages.`,
      `
`,
    ),
    createElement(paragraph, {}),
    createElement(
      code,
      { class: "language-js" },
      `<codeblock-js>String.prototype.ᐅ = function(f) {
  return f(this)
}
</codeblock-js>`,
    ),
    createElement(
      paragraph,
      {},
      `Note that it's a new method on JavaScript's String object. It could work the same on every object (even on Object).`,
      `
`,
    ),
    createElement(heading, { level: 2, id: "pb-todo" }, `Usage`),
    createElement(
      paragraph,
      {},
      `This is the same example as in the proposal.`,
      `
`,
    ),
    createElement(paragraph, {}),
    createElement(
      code,
      { class: "language-js" },
      `<codeblock-js>function doubleSay (str) {
  return str + ", " + str;
}
function capitalize (str) {
  return str[0].toUpperCase() + str.substring(1);
}
function exclaim (str) {
  return str + '!';
}

let result = "hello"
  .ᐅ (doubleSay)
  .ᐅ (capitalize)
  .ᐅ (exclaim)

result //=> "Hello, hello!"
</codeblock-js>`,
    ),
  );
}
