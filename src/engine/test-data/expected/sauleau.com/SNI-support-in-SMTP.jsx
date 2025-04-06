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
      paragraph,
      {},
      `While working on `,
      createElement(
        link,
        {
          style: {
            textDecoration: "underline",
            color: "rgb(6.666667%, 33.333336%, 80%)",
          },
          href: "https://mailway.app/",
        },
        `mailway.app`,
      ),
      ` I experimented with routing emails based on `,
      createElement(
        link,
        {
          style: {
            textDecoration: "underline",
            color: "rgb(6.666667%, 33.333336%, 80%)",
          },
          href: "https://en.wikipedia.org/wiki/Server_Name_Indication",
        },
        `SNI`,
      ),
      `.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Let’s imagine a common email setup:`,
      `
`,
    ),
    createElement(
      list,
      {},
      createElement(
        listItem,
        {},
        `sauleau.com has a MX record that points to mx.sauleau.com.`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        `mx.sauleau.com has an A record that points to an email server’s IP.`,
        `
`,
      ),
    ),
    createElement(
      paragraph,
      {},
      `When someone sends an email to hello@sauleau.com, it needs to resolve the `,
      createElement(
        link,
        {
          style: {
            textDecoration: "underline",
            color: "rgb(6.666667%, 33.333336%, 80%)",
          },
          href: "https://en.wikipedia.org/wiki/MX_record",
        },
        `MX record`,
      ),
      ` and the A record to find the corresponding email server.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Traditionally, mx.sauleau.com would run on its dedicated email server and listen on port 25. To run a secure email server we have a TLS certificate stored on the server issued specifically for mx.sauleau.com.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `The certificate is presented to anyone connecting to the IP and asking for a secure connection. Which implies that it’s not related to the hostname we used to resolve the IP (A record), more importantly, most clients will abort if the TLS certificate doesn’t match the hostname it attempted to connect to.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `To address this issue we could use `,
      createElement(
        link,
        {
          style: {
            textDecoration: "underline",
            color: "rgb(6.666667%, 33.333336%, 80%)",
          },
          href: "https://en.wikipedia.org/wiki/Server_Name_Indication",
        },
        `SNI`,
      ),
      ` since it appears earlier in the SMTP communication with the email server and allows to select a TLS certificate to present to the client, which would use the right hostname. However, not all email clients support sending the `,
      createElement(
        link,
        {
          style: {
            textDecoration: "underline",
            color: "rgb(6.666667%, 33.333336%, 80%)",
          },
          href: "https://en.wikipedia.org/wiki/Server_Name_Indication",
        },
        `SNI`,
      ),
      `.`,
      `
`,
    ),
    createElement(
      heading,
      { level: 2, id: "pb-todo" },
      `Email providers support`,
    ),
    createElement(paragraph, {}),
    createElement(
      table,
      {},
      createElement(
        tableRow,
        {},
        createElement(
          tableCell,
          {},
          createElement(
            "span",
            {
              style: {
                textAlign: "center",
                marginLeft: "auto",
                marginRight: "auto",
              },
            },
            `Name`,
          ),
        ),
        createElement(
          tableCell,
          {},
          createElement(
            "span",
            {
              style: {
                textAlign: "center",
                marginLeft: "auto",
                marginRight: "auto",
              },
            },
            `Supports SNI?`,
          ),
        ),
      ),
      createElement(
        tableRow,
        {},
        createElement(
          tableCell,
          {},
          createElement(
            link,
            {
              style: {
                textDecoration: "underline",
                color: "rgb(6.666667%, 33.333336%, 80%)",
              },
              href: "https://gmail.com/",
            },
            `Gmail`,
          ),
        ),
        createElement(tableCell, {}, `yes`),
      ),
      createElement(
        tableRow,
        {},
        createElement(
          tableCell,
          {},
          createElement(
            link,
            {
              style: {
                textDecoration: "underline",
                color: "rgb(6.666667%, 33.333336%, 80%)",
              },
              href: "https://mail.yahoo.com/",
            },
            `Yahoo`,
          ),
        ),
        createElement(tableCell, {}, `no`),
      ),
      createElement(
        tableRow,
        {},
        createElement(
          tableCell,
          {},
          createElement(
            link,
            {
              style: {
                textDecoration: "underline",
                color: "rgb(6.666667%, 33.333336%, 80%)",
              },
              href: "https://mail.aol.com/",
            },
            `AOL`,
          ),
        ),
        createElement(tableCell, {}, `no`),
      ),
      createElement(
        tableRow,
        {},
        createElement(
          tableCell,
          {},
          createElement(
            link,
            {
              style: {
                textDecoration: "underline",
                color: "rgb(6.666667%, 33.333336%, 80%)",
              },
              href: "https://protonmail.com/",
            },
            `ProtonMail`,
          ),
        ),
        createElement(tableCell, {}, `no`),
      ),
      createElement(
        tableRow,
        {},
        createElement(
          tableCell,
          {},
          createElement(
            link,
            {
              style: {
                textDecoration: "underline",
                color: "rgb(6.666667%, 33.333336%, 80%)",
              },
              href: "https://outlook.live.com/",
            },
            `Outlook`,
          ),
        ),
        createElement(tableCell, {}, `yes`),
      ),
      createElement(
        tableRow,
        {},
        createElement(
          tableCell,
          {},
          createElement(
            link,
            {
              style: {
                textDecoration: "underline",
                color: "rgb(6.666667%, 33.333336%, 80%)",
              },
              href: "https://www.mailgun.com/",
            },
            `Mailgun`,
          ),
        ),
        createElement(tableCell, {}, `yes`),
      ),
      createElement(
        tableRow,
        {},
        createElement(
          tableCell,
          {},
          `Golang’s `,
          createElement(
            link,
            {
              style: {
                textDecoration: "underline",
                color: "rgb(6.666667%, 33.333336%, 80%)",
              },
              href: "https://golang.org/pkg/net/smtp/",
            },
            `net/smtp`,
          ),
        ),
        createElement(tableCell, {}, `yes`),
      ),
    ),
    createElement(
      paragraph,
      {},
      `This list is far from exhaustive, feel free to reach out if you want to try a specific email provider.`,
      `
`,
    ),
  );
}
