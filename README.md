# autonyt <!-- omit in toc -->

`autonyt` is a collection of overengineered tools for solving [NYT Games](https://www.nytimes.com/crosswords).

## Table of contents <!-- omit in toc -->

- [`nyt-word-patch`](#nyt-word-patch)
  - [Usage](#usage)
- [`hexabee`](#hexabee)
  - [Usage](#usage-1)
  - [Design](#design)
    - [Approach 1: index and set intersection](#approach-1-index-and-set-intersection)
    - [Approach 2: generate valid words](#approach-2-generate-valid-words)
    - [Approach 3: grep](#approach-3-grep)
- [`unbox`](#unbox)
  - [Usage](#usage-2)
  - [Design](#design-1)
    - [Idea 1](#idea-1)
    - [Idea 2](#idea-2)

## `nyt-word-patch`

`nyt-word-patch` removes words from the English words dictionary provided by the git submodule at `third_party/english-words` at `words_alpha.txt` that NYT does not accept as words in their puzzle games. This prints the modified file to to STDOUT instead of modifying in-place.

### Usage

```bash
$ ./nyt-word-patch/nyt-word-patch.sh $PATH_TO_REMOVE_WORDS $FILE_TO_REMOVE_FROM > $OUTPUT_FILE

# For example:
$ ./nyt-word-patch/nyt-word-patch.sh ./nyt-word-patch/not-valid-puzzle-words.txt ./third_party/english-words/words_alpha.txt > .scratch/puzzle-words

# To verify:
$ sdiff -l ./.scratch/puzzle-words ./third_party/english-words/words_alpha.txt | cat -n | grep -v -e '($'
```

## `hexabee`

`hexabee` solves the Spelling Bee challenge.

### Usage

```bash
$ hexabee --letters $OTHER_LETTERS --center $CENTER_LETTER

# For example:
$ hexabee --letters abcdef --center g
```

### Design

- Let $\alpha$ be the center letter.
- Let $L$ be the set of other letters.
- Let $L'$ be $\{\alpha\} \cup L$.
- Let $C$ be the set of all English characters.
- Let $W$ be the set of all English words.
- Let $W_\alpha$ be the set of English words containing the character $\alpha$.

#### Approach 1: index and set intersection

- For all characters $c \in C$: construct an index of $c$ to $W_c$.
- Construct $W_L = \bigcup_{\alpha' \in L} W_{\alpha'}$.
- The answer is $W_\alpha \cap W_L$.

#### Approach 2: generate valid words

- Construct a prefix tree of all words, where each node is a character. At each node, annotate the set of characters reachable by successor paths in the tree.
  - This tree is at most 26 wide and 31 deep (the longest English word is 31 letters).
- Traverse the prefix tree from the root to generate letter permutations using the characters in $L'$:
  - Only traverse to nodes whose characters are in $L'$.
  - Stop traversing down paths when intersection of the set of reachable characters for that path and $L'$ is empty.
- For each generated letter permutation:
  - Check that it's a valid word. (Alternatively, only record generated letter permutations that are valid words - we want to avoid early stopping at partial words.)
  - Check that the word contains the center letter. (Alternatively, only record generated letter permutations that contain the center letter.)

#### Approach 3: grep

Apparently, just search the whole fucking list of words:

```bash
# To solve 2020-11-27:
$ rg '^[eplnovd]+\s' third_party/english-words/words_alpha.txt | sed -r '/^.{,4}$/d' | rg n | less

# In general:
$ rg "^[$ALL_LETTERS]+\s" third_party/english-words/words_alpha.txt | sed -r '/^.{,4}$/d' | rg $CENTER_LETTER | less
```

So much for solving interesting data structure problems.

## `unbox`

`unbox` solves the Letter Boxed challenge.

### Usage

```bash
$ unbox $TOP_LETTERS $RIGHT_LETTERS $BOTTOM_LETTERS $LEFT_LETTERS

# For example:
$ unbox uoa qtl ein ysm
```

### Design

#### Idea 1

- Generate English prefix tree.
- From each character in each side, attempt to traverse English prefix tree to another character on another side - generate all possible words this way.
- Find aligning words (first character of one word is last character of another), and find shortest aligning sequence that contains all letters.

#### Idea 2

- Scan list of English words.
- Filter list to words that can be generated by box.
- Find aligning words.
