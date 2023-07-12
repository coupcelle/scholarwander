
# Design Decisions

## Dependencies

### Openssl
This is what the original securew2 project used and we were initially lazy - this could probably be replaced with some kind of [cms](https://crates.io/crates/cms) implementation


## networkmanager
There seemed to really only be one crate for hooking into network manager


## wry/web libraries

This seemed to be one of the view "display a custom browser window" dependencies available in rust (which is needed in order to be able to extract the token from the loaded page at the end).

The other alternative looked to be lighter weight but hasnt been maintained in the 6-8 months prior and i wasnt really sure if any of the current issues would affect our usecase.

Also the docs didnt seem particularly clear as to whether it could load webpages.

### XML parser: serde-xml-rs
also seems kinda unmaintained but also serde seems like the most reasonable choice.

Given the weird double nested structure of our config files, we end up with a ton of intermediate structss that arent needed. this feature would help but it hasnt been merged and its a bit broken.
https://github.com/RReverser/serde-xml-rs/pull/121
