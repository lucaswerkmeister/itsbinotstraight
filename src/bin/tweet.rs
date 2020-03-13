use rust_twitter_bot_lib::TwitterBot;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let bot = TwitterBot::new()
        .consumer_key(&dotenv::var("TWITTER_CK")?)
        .consumer_secret_key(&dotenv::var("TWITTER_CS")?)
        .access_token(&dotenv::var("TWITTER_TK")?)
        .secret_access_token(&dotenv::var("TWITTER_TS")?);

    let mut rng = rand::thread_rng();
    let biword = itsbinotstraight::biword(&mut rng);
    let phrase = itsbinotstraight::phrase(&biword, &mut rng);

    let tweet = bot.tweet(&phrase, None)?;
    println!("https://twitter.com/status/status/{}", tweet.id());
    Ok(())
}
