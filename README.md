# wordlesolver

wordle solver

Execute with...
  **cargo run --release play**
  
wordlesolver will respond with a guess. After you enter the guess in wordle, enter the clue you got back.

To enter the clue, use " " for missed letters, "Y" for yellow letters and "G" for green. The clue you enter should look
something like "  Y  " or " G  Y"


There are a few other options that just calculate stats for the words

Wordlesovler uses [Shanon Entropy](https://en.wikipedia.org/wiki/Entropy_(information_theory)) to rank the possible words. As such, "SOARE" is the first guess. A few other ways of ranking the words are built in, but commented out. From experience, Shanon Entropy is slightly better than the others.

My next goal is to evaulate that rigorusly.

