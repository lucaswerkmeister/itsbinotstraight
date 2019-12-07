const fs = require('fs').promises;

async function biword() {
    const contents = await fs.readFile('biwords', 'utf8'),
          biwords = contents.split('\n');
    biwords.pop(); // remove trailing newline
    const index = Math.floor(Math.random() * biwords.length);
    return biwords[index];
}

async function tweet() {
    const word = await biword();
    return `it’s ${word} not ${word.replace(/bi/i, 'straight')}`;
}

module.exports = tweet;

if (require.main === module) {
    tweet().then(console.log, console.error);
}
