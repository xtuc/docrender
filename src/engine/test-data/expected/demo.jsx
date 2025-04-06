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
    createElement(
      heading,
      {
        level: 1,
        style: { textAlign: "center", marginLeft: "auto", marginRight: "auto" },
        id: "pb-todo",
      },
      `Turn any document`,
    ),
    createElement(
      heading,
      {
        level: 1,
        style: { textAlign: "center", marginLeft: "auto", marginRight: "auto" },
        id: "pb-todo",
      },
      `into a website`,
    ),
    createElement(paragraph, {}),
    createElement(
      heading,
      { level: 3, id: "pb-todo" },
      `Publish Bot allows you to turn any document into a website.`,
    ),
    createElement(
      paragraph,
      {},
      `This is a demo. `,
      createElement("span", { style: { color: "rgb(100%, 0%, 0%)" } }, `M`),
      createElement(
        "span",
        { style: { color: "rgb(100%, 60.000004%, 0%)" } },
        `o`,
      ),
      createElement(
        "span",
        { style: { color: "rgb(60.000004%, 0%, 100%)" } },
        `s`,
      ),
      createElement(
        "span",
        { style: { color: "rgb(15.294118%, 30.588236%, 7.4509807%)" } },
        `t`,
      ),
      ` of the Google Docs `,
      createElement("span", { style: { fontStyle: "italic" } }, `features`),
      ` are `,
      createElement("span", { style: { fontWeight: "bold" } }, `supported`),
      ` 😀.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `And code blocks:`,
      `
`,
    ),
    createElement(
      code,
      { class: "language-js" },
      `<codeblock-js>console.log(“code example”)
</codeblock-js>`,
    ),
    createElement(
      paragraph,
      {},
      createElement(image, {
        style: { textAlign: "center", marginLeft: "auto", marginRight: "auto" },
        src: "ed9dc5e3980bcdbcb51a29458192ca155592e122affef1bf7bf11e9e7012bce3",
        height: "67",
        width: "66",
      }),
    ),
    createElement(paragraph, {}),
    createElement(
      paragraph,
      {},
      `See the source Google Docs: `,
      createElement(
        link,
        {
          style: {
            textDecoration: "underline",
            color: "rgb(6.666667%, 33.333336%, 80%)",
          },
          href: "https://docs.google.com/document/d/1nSn4UBI-xo0tE5PwJOq7Dp3TgLlhT7b-dM3rK2xcHsk/edit?usp=sharing",
        },
        `https://docs.google.com/document/d/1nSn4UBI-xo0tE5PwJOq7Dp3TgLlhT7b-dM3rK2xcHsk/edit?usp=sharing`,
      ),
    ),
    createElement(paragraph, {}),
  );
}
