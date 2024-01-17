Overview
This Rust-based parser experiment, with progress made up to episode 6 of Timothy Laceby's "A Guide to Interpreters" series. 

Continuous testing during development:

cargo watch -q -c -w src/ -x "test"

Automatically run the project when source files change:

cargo watch -q -c -w src/ -x "run"


Acknowledgements
Inspired by Timothy Laceby's "A Guide to Interpreters" series (up to episode 6). https://www.youtube.com/playlist?list=PL_2VhOvlMk4UHGqYCLWc6GO8FaPl8fQTh
Lexical analysis using the logos crate. https://github.com/maciejhirsz/logos