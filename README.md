# Word Finder

A CLI tool that finds words of a specific length with letters in a specific location

I use this for building crosswords

https://github.com/user-attachments/assets/e3393b22-b91d-45f2-ac16-19c11be40fc2

## Installation

### Build from Source

Clone the repository and build the project using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```sh
git clone https://github.com/Talon-Games/word-finder
cd word-finder
cargo build --release
# The binary will be located at target/release/word-finder
```

To add the binary to your PATH, run:

```sh
cargo install --path .
```

### Pre-built Binaries

Pre-built binaries are available for Windows, macOS, and Linux on the [releases page](https://github.com/Talon-Games/word-finder/releases).

## Usage

After building or downloading the tool, you can start using Word Finder by running the following command in your terminal:

```sh
word-finder [word list]
```

- If you don't provide any arguments, the tool will default to searching for English words.
- To search for Latin words, run the tool with the argument `latin`:

```sh
word-finder latin
```

- To use a custom word list enter a relative path to a text file

```sh
word-finder ./custom-words.txt
```

custom-words.txt

```txt
word ::
word :: with definition
```

### Interactive Search

Once the tool is running, you'll be prompted to:

1. **Enter the word length**: Specify the number of letters in the word you're looking for (between 1 and 50).
2. **Define letter positions**: For each position in the word, you can either:
   - Enter a letter to require at that specific location
   - Enter a '-' to require a consonant at that specific location
   - Enter a '\*' to require a vowel at that specific location
   - Leave it blank by pressing Enter if you don't want to restrict that position to a specific letter.

### Example

For instance, if you're looking for a 5-letter word with 'a' as the second letter and 'e' as the fourth letter, you would follow these steps:

```sh
Word length: 5
1st letter: [leave blank]
2nd letter: a
3rd letter: [leave blank]
4th letter: e
5th letter: [leave blank]
```

The program will then search through the word list and display matching words along with their definitions.

Press `esc` at any time to exit the program.
