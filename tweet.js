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

async function tweet() {
    const word = await biword(),
          replacement = word.replace(/bi/i, match => replacements[match][0]);
    return `it’s ${word} not ${replacement}`;
}

module.exports = tweet;

if (require.main === module) {
    tweet().then(console.log, console.error);
}
