import Prism from 'prismjs';

export function highlight(text, lang) {
    let grammar = Prism.languages.text;

    if (Prism.languages[lang.toLowerCase()] !== undefined) {
        grammar = Prism.languages[lang.toLowerCase()];
    }

    return Prism.highlight(text, grammar, lang)
}
