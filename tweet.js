const fs = require('fs').promises;
const process = require('process');

const replacements = {
    bi: ['straight', 'hetero'],
    Bi: ['Straight', 'Hetero'],
    BI: ['STRAIGHT', 'HETERO'],
    bI: ['sTrAiGhT', 'hEtErO'],
};
const patterns = [
    // chance that this or one of the preceding patterns will be picked, pattern/phrase, optional conversion for biword and replacement
    [0.005, 'IT’S 👏 %BIWORD% 👏 NOT 👏 %REPLACEMENT% 👏', (s) => s.toUpperCase()],
    [0.015, 'IT’S %BIWORD% NOT %REPLACEMENT%', (s) => s.toUpperCase()],
    [0.05, 'you must understand: it’s %BIWORD%, not %REPLACEMENT%', null],
    [0.1, 'after all, it’s %BIWORD% not %REPLACEMENT%', null],
    [0.2, 'you know what they say, it’s %BIWORD% not %REPLACEMENT%', null],
    [0.3, 'as you know, it’s %BIWORD%, not %REPLACEMENT%', null],
    [0.4, 'don’t forget – it’s %BIWORD%, not %REPLACEMENT%', null],
    [0.5, 'always remember that it’s %BIWORD%, not %REPLACEMENT%', null],
    [0.6, 'heads up it’s called %BIWORD% and not %REPLACEMENT%', null],
    [0.7, 'it’s “%BIWORD%”\nnot “%REPLACEMENT%”', null],
    [0.85, 'it’s %BIWORD% and not %REPLACEMENT%', null],
    [1, 'it’s %BIWORD% not %REPLACEMENT%', null],
];
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
    biword = biword.normalize('NFD'); // ensure that e. g. Bì… can still be matched by regexp below
    const [_, bi, nextLetter] = biword.match(/(bi)(.?)/i),
          chanceForConsonant = /[aeiouy]/.test(nextLetter) ? 0.7 : 0.3,
          chooseConsonant = Math.random() < chanceForConsonant;
    let replacement = biword.replace(bi, replacements[bi][chooseConsonant ? 0 : 1]);
    for (const regexp of [/(straight)t/i, /(hetero)o/i]) {
        replacement = replacement.replace(regexp, (_, part, doubledLetter) => part);
    }
    replacement = replacement.normalize('NFC'); // undo NFD normalization above
    return replacement;
}

function phrase(biword) {
    const sample = Math.random();
    let pattern, conversion;
    for (const [chance, pattern_, conversion_] of patterns) {
        if (sample < chance) {
            pattern = pattern_;
            conversion = conversion_;
            break;
        }
    }
    if (pattern === undefined) {
        throw new Error(`No pattern for sample ${sample}!`);
    }
    if (!conversion) {
        conversion = (s) => s;
    }
    return pattern.replace('%BIWORD%', conversion(biword))
        .replace('%REPLACEMENT%', conversion(replacement(biword)));
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
    main().catch((e) => { console.error(e); process.exitCode = 1; });
}
