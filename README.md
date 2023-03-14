# Wordle
Wordle CLI-game made in rust.

<img width="407" alt="image" src="https://user-images.githubusercontent.com/43318657/222103546-13868543-35be-4c75-a203-14c070686007.png">

## How to compile?
```
git clone https://github.com/oliverlaursen/wordle.git
cd wordle
cargo run
```

## Change the config
If you compiled yourself, you can run 
```
cargo run -- -help
```
to see what arguments can be passed. 

## Languages
The currently supported languages are english and danish.
To change the language to danish run
```
cargo run -- -l danish
```
This will change the language the messages are in, the language in which the word is generated in and the language in which words are allowed.

## Adding a language
To add a language, add the following 3 files 
- languages/\<LANGUAGE\>.txt 
  - This should have a translation for each message the game has, see languages/english.txt for guideline
- wordlists/wordlist_\<LANGUAGE\>.txt
  - This should hold the words that can be generated for the given language (Something like top 3000 most popular words)
- wordlists/wordlistfull\<LANGUAGE\>.txt
  - This should hold all words in the given language and is used for checking whether a guess is a real word
