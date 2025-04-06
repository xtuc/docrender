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
    createElement(heading, { level: 2, id: "pb-todo" }, `Simple program`),
    createElement(
      paragraph,
      {},
      `The example project uses the following files:`,
      `
`,
    ),
    createElement(paragraph, {}),
    createElement(heading, { level: 3, id: "pb-todo" }, `secret/secret.go:`),
    createElement(paragraph, {}),
    createElement(
      code,
      { class: "language-go" },
      `<codeblock-js>package secret

import "log"

func ILoveTrains() {
    log.Println("secret")
}
</codeblock-js>`,
    ),
    createElement(paragraph, {}),
    createElement(heading, { level: 3, id: "pb-todo" }, `cmd/test/main.go:`),
    createElement(paragraph, {}),
    createElement(
      code,
      { class: "language-go" },
      `<codeblock-js>package main

import "github.com/xtuc/example/secret"

func main() {
    secret.ILoveTrains()
}
</codeblock-js>`,
    ),
    createElement(heading, { level: 2, id: "pb-todo" }, `Disassembly`),
    createElement(
      paragraph,
      {},
      `You actually don't need to disassemble the binary. For the sake of simplicity, I'll only use strings against the binary.`,
      `
`,
    ),
    createElement(paragraph, {}),
    createElement(
      code,
      { class: "language-bash" },
      `<codeblock-js>$ cd cmd/test
$ go build
$ strings ./test | grep secret

github.com/xtuc/example/secret.ILoveTrains
[…]
</codeblock-js>`,
    ),
    createElement(
      paragraph,
      {},
      `Note that Go won't include can that you don't use. All the imports are used.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `We can clearly see that github.com/xtuc is loving trains and even see the file structure. It makes no sense to include such information in the final binary, it's what we call a disclosure.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `You might have noticed that I didn't stripped the binary. Let's try again with a stripped binary.`,
      `
`,
    ),
    createElement(paragraph, {}),
    createElement(
      code,
      { class: "language-bash" },
      `<codeblock-js>$ cd cmd/test
$ go build -ldflags="-s -w"
$ file ./test
test: ELF 64-bit LSB executable […] stripped
$ strings ./test | grep secret

github.com/xtuc/example/secret.ILoveTrains
[…]
</codeblock-js>`,
    ),
    createElement(
      paragraph,
      {},
      `Even with a stripped (it's meant for distribution) we can still see them.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Since the binary is stripped we don't have debugging symbols anymore:`,
      `
`,
    ),
    createElement(paragraph, {}),
    createElement(
      code,
      { class: "language-bash" },
      `<codeblock-js>$ nm ./test
nm: ./test: no symbols
</codeblock-js>`,
    ),
    createElement(
      heading,
      { level: 2, id: "pb-todo" },
      `Stack Overflow, what's the solution?`,
    ),
    createElement(
      paragraph,
      {},
      `Stack Overflow can be useful in many cases, but not this time. This is the answer I read:`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Why do you need even that?`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Don't distribute Golang binaries.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Definitely not helpful.`,
      `
`,
    ),
    createElement(paragraph, {}),
    createElement(
      heading,
      { level: 2, id: "pb-todo" },
      `Golang, what's the solution?`,
    ),
    createElement(
      paragraph,
      {},
      `I didn't ask, so this is not their answer: as many things in Golang, it's highly opinionated and specific to Google's needs.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Since they don't distribute any Go binary, why would they invest time in an obfuscation process.`,
      `
`,
    ),
    createElement(paragraph, {}),
    createElement(
      heading,
      { level: 2, id: "pb-todo" },
      `What's the real solution?`,
    ),
    createElement(
      paragraph,
      {},
      `Since the Golang compiler won't mangle function names, I decided to mangle them before passing it to the compiler.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `It's a source mangler: `,
      createElement(
        link,
        {
          style: {
            textDecoration: "underline",
            color: "rgb(6.666667%, 33.333336%, 80%)",
          },
          href: "https://github.com/xtuc/manglo",
        },
        `https://github.com/xtuc/manglo`,
      ),
      `.`,
    ),
    createElement(paragraph, {}),
    createElement(paragraph, {}),
    createElement(
      heading,
      { level: 2, id: "pb-todo" },
      `Alternative compiler backends`,
    ),
    createElement(
      paragraph,
      {},
      `My example uses the default Go compiler. I tried the gccgo compiler (GCC with a Go fronted) but it refused to compile because the imports weren't quite the same (might have changed somewhere around Go1.9).`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `GCC mangles all names.`,
      `
`,
    ),
    createElement(paragraph, {}),
    createElement(heading, { level: 2, id: "pb-todo" }, `Packing?`),
    createElement(
      paragraph,
      {},
      `Packing the binary will indeed removing the clear strings.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Anyone having disassemble skills will notice it immediately and revert the process (unpack).`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `I'm not sure if it works cross-platform.`,
      `
`,
    ),
    createElement(paragraph, {}),
    createElement(paragraph, {}),
  );
}
