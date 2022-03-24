# ashtray

An Anagramhay and Yahmargana discoverer.

## Table of Contents

- [Project Description](#project-description)
- [Background](#background)
  - [What is an Anagramhay](#what-is-an-anagramhay)
  - [What is an Yahmargana](#what-is-an-yahmargana)
- [Methodology](#methodology)
- [Findings](#findings)

## Project Description

Finding Anagramhays and Yahmarganas is wicked hard and there has never been an attempt to find a comprehensive list of these words until NOW!

## Background

### What is an Anagramhay?

An **Anagramhay** is an Anagram where a word's Pig Latin translation is another word.

For example:

- **ashtray** is **trash** in Pig Latin
- **overlay** is **lover** in Pig Latin

### What is a Yahmargana?

A **Yahmargana** is like an **Anagramhay**, but you reverse the order of a word to make it into a **Anagramha**.

There are 3 types of Yahmarganas:

1. The original word is translated into Pig Latin, then this translation is reversed
   - e.g. **knee** -> Pig Latin -> **eeknay** -> Reversed -> **yankee**
2. The original word is reversed, then translated into Pig Latin
   - e.g. **all** -> Reversed -> **lla** -> Pig Latin -> **allay**
3. Combination of 1 & 2 -- the original word is reversed, translated into Pig Latin, then the Pig Latin is revered
   - e.g. **keen** -> Reversed -> **neek** -> Pig Latin -> eeknay -> Reversed -> **yankee**

## Methodology

I put together a list of ~376k English words from various free wordlists sources.
I then created a CLI program _ashtray_ which checks for any Anagramhays or Yahmarganas matches within a set of words.

## Findings

The initial results showed about 100 Anagramhays and Yahmarganas out of the 376k word list. However some of these matched words will get you smacked if you tried to play them in Scrabble -- theay are just acronyms, abbreviations, proper nouns, and old-timey spellings of other words and so they aren't as satisfying as some of the other _wordy_ words.

So, I added an additional filter which cross-checks the matches with Marriam-Webster's online dictionary and only words that are either a noun, adjective, or verb are included in the final `matches.txt`. There are still some 1-2 letter words that aren't common words so if you exclude these, you get the following list:

### Anagramhays

| Word    | Pig Latin |
| ------- | --------- |
| ko      | okay      |
| sun     | unsay     |
| tres    | estray    |
| raff    | affray    |
| rest    | estray    |
| lout    | outlay    |
| stout   | outstay   |
| touts   | outstay   |
| trash   | ashtray   |
| lover   | overlay   |
| plover  | overplay  |
| stover  | overstay  |
| linter  | interlay  |
| plunder | underplay |

### Yahmarganas (type 1)

| Word | Pig Latin | nitaL giP |
| ---- | --------- | --------- |
| hoo  | oohay     | yahoo     |
| knee | eeknay    | yankee    |

### Yahmarganas (type 2)

| Word | droW | Pig Latin |
| ---- | ---- | --------- |
| ok   | ko   | okay      |
| nil  | lin  | inlay     |
| ess  | sse  | essay     |
| nus  | sun  | unsay     |
| all  | lla  | allay     |

### Yahmarganas (type 3)

| Word | droW | Pig Latin | nitaL giP |
| ---- | ---- | --------- | --------- |
| ooh  | hoo  | oohay     | yahoo     |
| rant | tnar | artnay    | yantra    |
| keen | neek | eeknay    | yankee    |
