const process = require('process')
const Twitter = require('twitter');

const client = new Twitter({
    consumer_key: process.env.TWITTER_CK,
    consumer_secret: process.env.TWITTER_CS,
    access_token_key: process.env.TWITTER_TK,
    access_token_secret: process.env.TWITTER_TS,
});

async function sendTweet() {
    try {
        const tweet = await client.post('statuses/update', { status: 'it’s bi not straight' });
        console.log(tweet);
    } catch (e) {
        console.error(e);
        process.exitCode = 1;
    }
}

sendTweet();
