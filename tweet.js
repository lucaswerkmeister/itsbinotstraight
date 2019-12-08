const fs = require('fs').promises;
const process = require('process');

const replacements = {
    bi: ['straight', 'hetero'],
    Bi: ['Straight', 'Hetero'],
    BI: ['STRAIGHT', 'HETERO'],
    bI: ['sTrAiGhT', 'hEtErO'],
};
let biwords = undefined;

async function loadBiwords() {
    const contents = await fs.readFile('biwords', 'utf8'),
          biwords = contents.split('\n');
    biwords.pop(); // remove trailing newline
    return biwords;
}

async function biword() {
    const words = await (biwords || (biwords = loadBiwords())),
          index = Math.floor(Math.random() * words.length);
    return words[index];
}

function replacement(biword) {
    const [_, bi, nextLetter] = biword.match(/(bi)(.)/i),
          chanceForConsonant = /[aeiouy]/.test(nextLetter) ? 0.7 : 0.3,
          chooseConsonant = Math.random() < chanceForConsonant;
    let replacement = biword.replace(bi, replacements[bi][chooseConsonant ? 0 : 1]);
    for (const regexp of [/(straight)t/i, /(hetero)o/i]) {
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
}

module.exports = tweet;

async function main() {
    let count = 1;
    if (process.argv.length === 3) {
        const arg = parseInt(process.argv[2], 10);
        if (!isNaN(arg)) {
            count = arg;
        }
    }
    for (let i = 0; i < count; i++) {
        console.log(await tweet());
    }
}

if (require.main === module) {
    main().catch((e) => { console.error(e); processe.exitCode = 1; });
}
