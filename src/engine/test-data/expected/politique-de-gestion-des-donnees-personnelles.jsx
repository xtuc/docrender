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
      { level: 3, id: "pb-todo" },
      `Politique de gestion des données personnelles`,
    ),
    createElement(
      paragraph,
      {},
      createElement(
        "span",
        { style: { fontWeight: "bold" } },
        `[Nom de l'entreprise/organisation]`,
      ),
      ` s'engage à protéger la vie privée de ses utilisateurs et à traiter leurs données personnelles dans le respect des lois et réglementations en vigueur, notamment le Règlement Général sur la Protection des Données (RGPD).`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      createElement(
        "span",
        { style: { fontWeight: "bold" } },
        `1. Données collectées`,
      ),
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Nous collectons les données suivantes :`,
      `
`,
    ),
    createElement(
      list,
      {},
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Données d'identification :`,
        ),
        `
`,
        ` nom, prénom, adresse email, numéro de téléphone, etc.`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Données de connexion :`,
        ),
        `
`,
        ` adresse IP, type de navigateur, système d'exploitation, etc.`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Données de navigation :`,
        ),
        `
`,
        ` pages visitées, liens cliqués, durée de la visite, etc.`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Données relatives aux transactions :`,
        ),
        `
`,
        ` historique des achats, informations de paiement, etc. (si applicable)`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Toute autre donnée que vous nous fournissez volontairement.`,
        ),
        `
`,
      ),
    ),
    createElement(
      paragraph,
      {},
      createElement(
        "span",
        { style: { fontWeight: "bold" } },
        `2. Finalités du traitement`,
      ),
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Vos données sont collectées et traitées pour les finalités suivantes :`,
      `
`,
    ),
    createElement(
      list,
      {},
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Fournir et améliorer nos services :`,
        ),
        `
`,
        ` répondre à vos demandes, personnaliser votre expérience, etc.`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Gérer votre compte utilisateur :`,
        ),
        `
`,
        ` créer et gérer votre compte, traiter vos commandes, etc. (si applicable)`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Vous contacter :`,
        ),
        `
`,
        ` répondre à vos questions, vous envoyer des informations importantes, etc.`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Analyser l'utilisation de nos services :`,
        ),
        `
`,
        ` améliorer nos services, développer de nouvelles fonctionnalités, etc.`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Respecter nos obligations légales et réglementaires.`,
        ),
        `
`,
      ),
    ),
    createElement(
      paragraph,
      {},
      createElement(
        "span",
        { style: { fontWeight: "bold" } },
        `3. Base légale du traitement`,
      ),
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Le traitement de vos données repose sur les bases légales suivantes :`,
      `
`,
    ),
    createElement(
      list,
      {},
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Votre consentement :`,
        ),
        `
`,
        ` pour certaines opérations de traitement, nous vous demanderons votre consentement explicite.`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `L'exécution d'un contrat :`,
        ),
        `
`,
        ` pour traiter vos commandes et gérer votre compte utilisateur (si applicable).`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Le respect d'une obligation légale :`,
        ),
        `
`,
        ` pour répondre à nos obligations légales et réglementaires.`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Notre intérêt légitime :`,
        ),
        `
`,
        ` pour améliorer nos services et vous proposer des offres pertinentes.`,
        `
`,
      ),
    ),
    createElement(
      paragraph,
      {},
      createElement(
        "span",
        { style: { fontWeight: "bold" } },
        `4. Destinataires des données`,
      ),
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Vos données peuvent être transmises aux destinataires suivants :`,
      `
`,
    ),
    createElement(
      list,
      {},
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Nos employés et sous-traitants :`,
        ),
        `
`,
        ` uniquement dans la mesure nécessaire à l'accomplissement de leurs tâches.`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Nos partenaires commerciaux :`,
        ),
        `
`,
        ` avec votre consentement préalable.`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Les autorités compétentes :`,
        ),
        `
`,
        ` en cas d'obligation légale.`,
        `
`,
      ),
    ),
    createElement(
      paragraph,
      {},
      createElement(
        "span",
        { style: { fontWeight: "bold" } },
        `5. Durée de conservation`,
      ),
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Vos données sont conservées pendant la durée nécessaire à la réalisation des finalités pour lesquelles elles ont été collectées, et en tout état de cause dans le respect des délais légaux de conservation.  `,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      createElement(
        "span",
        { style: { fontWeight: "bold" } },
        `6. Sécurité des données`,
      ),
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Nous mettons en œuvre des mesures techniques et organisationnelles appropriées pour garantir la sécurité de vos données et prévenir tout accès non autorisé, toute modification, divulgation ou destruction.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      createElement("span", { style: { fontWeight: "bold" } }, `7. Vos droits`),
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Vous disposez des droits suivants concernant vos données personnelles :`,
      `
`,
    ),
    createElement(
      list,
      {},
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Droit d'accès :`,
        ),
        `
`,
        ` vous pouvez accéder aux données que nous détenons sur vous.`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Droit de rectification :`,
        ),
        `
`,
        ` vous pouvez demander la correction des données inexactes ou incomplètes.`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Droit à l'effacement :`,
        ),
        `
`,
        ` vous pouvez demander la suppression de vos données dans certaines circonstances.`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Droit à la limitation du traitement :`,
        ),
        `
`,
        ` vous pouvez demander la limitation du traitement de vos données dans certaines circonstances.`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Droit à la portabilité des données :`,
        ),
        `
`,
        ` vous pouvez recevoir vos données dans un format portable.`,
        `
`,
      ),
      createElement(
        listItem,
        {},
        createElement(
          "span",
          { style: { fontWeight: "bold" } },
          `Droit d'opposition :`,
        ),
        `
`,
        ` vous pouvez vous opposer au traitement de vos données dans certaines circonstances.`,
        `
`,
      ),
    ),
    createElement(
      paragraph,
      {},
      `Pour exercer vos droits, vous pouvez nous contacter à l'adresse email suivante : `,
      createElement(
        "span",
        { style: { fontWeight: "bold" } },
        `[adresse email]`,
      ),
      `.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      createElement("span", { style: { fontWeight: "bold" } }, `8. Cookies`),
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Nous utilisons des cookies pour améliorer votre expérience sur notre site web. Vous pouvez configurer votre navigateur pour refuser les cookies, mais cela pourrait affecter certaines fonctionnalités du site.  `,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      createElement(
        "span",
        { style: { fontWeight: "bold" } },
        `9. Modifications de la politique`,
      ),
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Nous nous réservons le droit de modifier cette politique de gestion des données personnelles à tout moment. Les modifications seront publiées sur notre site web.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      createElement("span", { style: { fontWeight: "bold" } }, `10. Contact`),
      `
`,
    ),
    createElement(
      paragraph,
      {},
      `Pour toute question relative à cette politique, vous pouvez nous contacter à l'adresse email suivante : `,
      createElement(
        "span",
        { style: { fontWeight: "bold" } },
        `[adresse email]`,
      ),
      `.`,
      `
`,
    ),
    createElement(
      paragraph,
      {},
      createElement(
        "span",
        { style: { fontWeight: "bold" } },
        `[Date de la dernière mise à jour]`,
      ),
    ),
    createElement(paragraph, {}),
    createElement(paragraph, {}),
  );
}
