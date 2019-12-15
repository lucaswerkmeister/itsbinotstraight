require('dotenv').config();
const fs = require('fs').promises;
const process = require('process')
const Twitter = require('twitter');
const tweet = require('./tweet');

async function sendTweet() {
    const client = new Twitter({
        consumer_key: process.env.TWITTER_CK,
        consumer_secret: process.env.TWITTER_CS,
        access_token_key: process.env.TWITTER_TK,
        access_token_secret: process.env.TWITTER_TS,
    });

    const result = await client.post('statuses/update', { status: await tweet() });
    console.log(`https://twitter.com/status/status/${result.id_str}`);
}

(async function() {
    try {
        await sendTweet();
    } catch (e) {
        console.error(e);
        process.exitCode = 1;
    }
})();
