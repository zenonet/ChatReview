# Chat Review

Chat review is a platform for peer-reviewing conversations. Think of subreddits like r/AITAH or r/AmIOverreacting.
You can publish conversations you've had and other people can rate messages and comment on them.
Hopefully, this might help socially insecure people to be a little more confident when speaking.

## Structure

The webapp is has 3 main parts:
- A REST API written in Rust using axum
- A postgresql database
- A vue.js 3 frontend packed using rsbuild