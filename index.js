const fs = require('fs').promises;
const process = require('process')
const Twitter = require('twitter');
const tweet = require('./tweet');

const env = { ...process.env };

async function readEnvFile(path) {
    let contents;
    try {
        contents = await fs.readFile(path, 'utf8');
    } catch (e) {
        if (typeof e === 'object' && e.code === 'ENOENT') {
            return;
        } else {
            throw e;
        }
    }
    const assignments = contents.split('\n');
    assignments.pop(); // remove trailing newline
    for (const [name, value] of assignments.map(assignment => assignment.split('='))) {
        env[name] = value;
    }
}

async function sendTweet() {
    const client = new Twitter({
        consumer_key: env.TWITTER_CK,
        consumer_secret: env.TWITTER_CS,
        access_token_key: env.TWITTER_TK,
        access_token_secret: env.TWITTER_TS,
    });

    const result = await client.post('statuses/update', { status: await tweet() });
    console.log(`https://twitter.com/status/status/${result.id_str}`);
}

(async function() {
    try {
        await readEnvFile('.env');
        await sendTweet();
    } catch (e) {
        console.error(e);
        process.exitCode = 1;
    }
})();
