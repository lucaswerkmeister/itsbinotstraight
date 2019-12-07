const fs = require('fs').promises;

const replacements = {
    bi: ['straight', 'hetero'],
    Bi: ['Straight', 'Hetero'],
    BI: ['STRAIGHT', 'HETERO'],
    bI: ['sTrAiGhT', 'hEtErO'],
};

async function biword() {
    const contents = await fs.readFile('biwords', 'utf8'),
          biwords = contents.split('\n');
    biwords.pop(); // remove trailing newline
    const index = Math.floor(Math.random() * biwords.length);
    return biwords[index];
}

function replacement(biword) {
    const [_, bi, nextLetter, rest] = biword.match(/(^bi)(.)(.*$)/i),
          chanceForConsonant = /[aeiouy]/.test(nextLetter) ? 0.7 : 0.3,
          chooseConsonant = Math.random() < chanceForConsonant;
    let replacement = replacements[bi][chooseConsonant ? 0 : 1] + nextLetter + rest
    for (const regexp of [/(^straight)t/i, /(^hetero)o/i]) {
        replacement = replacement.replace(regexp, (_, part, doubledLetter) => part);
    }
    return replacement;
}

function phrase(biword) {
    const chanceForAllCaps = /^\p{Lu}*$/u.test(biword) ? 0.5 : 0,
          useAllCaps = Math.random() < chanceForAllCaps;
    if (useAllCaps) {
        return `IT’S ${biword} NOT ${replacement(biword)}`;
    } else {
        return `it’s ${biword} not ${replacement(biword)}`;
    }
}

async function tweet() {
    const word = await biword();
    return phrase(word);
    if (/^\p{Lu}*$/u.test(word)) {
        return `IT’S ${word} NOT ${replacement(word)}`;
    } else {
        return `it’s ${word} not ${replacement(word)}`;
    }
}

module.exports = tweet;

if (require.main === module) {
    tweet().then(console.log, console.error);
}
