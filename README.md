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
This will change the language the messages are in, and the language in which the word is generated in.

## Adding a language
To add a language, make a new .txt file in the languages folder (see english.txt for guideline). The language-file must be named \<LANGUAGE\>.txt
You will also need to add a wordlist to the wordlists folder, containing words you wish to generate. The wordlist-file must be named wordlist_\<LANGUAGE\>.txt
