# DW German Audio Downloader

This is a supplementary program to help you in your German language studies.
It is to be used with the (excellent) Deutsche Welle free German Language Learning resources that are found at https://learngerman.dw.com/en/overview

It has been tested with the Nico's Weg course. It works by being fed the URL of a vocabulary list (that is provided at the end of a section) or a whole 
course list (eg. course for A1) and provides you with options to download the pronunciation files (which are clear, concise and presented in a regular 
pattern throughout the course).
The pronunciation files can then be used with a supplementary learning aid, such as the creation of personal flash cards (using a free program such as Anki).

(This code is a rewrite in Rust of my Python script. It originally came about because after each chapter I found myself digging through the code so I could 
get the pronunciation files that were used to teach new words and to add them to my personal German flashcards. This code just saves a bit of time.)

## Requirements

This script uses the following crates:

* scraper = "0.13.0"
* reqwest = { version = "0.11", features = ["blocking", "json"] }
* text-colorizer = "1"

It has been tested on Linux (should work on Windows / Mac if the requirements are met), with the A1 Nico's Weg course (should work with other levels if the files are presented the same way).

## Usage

The code itself is very simple, therefore its usage should be very simple!

    $ cargo run -- URL download_type output_folder

It accepts the following positional arguments:

    URL                           URL of a vocabulary page from
                                  https://learngerman.dw.com (Nico's Weg)
    download_type                 c - Course List Download  (Loop Through List)
                                  p - Page Download         (Specific Page)
    output_folder                 Output Folder name        (Relative to Directory)

## Examples

### Download Course List

    $ cargo run -- https://learngerman.dw.com/en/nicos-weg/c-36519789 c A1 
    Url: /en/hallo/l-37250531
    Url: /en/kein-problem/l-37251054
    Url: /en/tschüss/l-37251033
    Url: /en/von-a-bis-z/l-37256418
    ...
    Word: Frau
    Saved File: Frau
    Word: nicht
    Saved File: nicht
    Word: oder
    Saved File: oder
    Word: und
    Saved File: und
    ...

### Download Specific Page

    $ cargo run -- https://learngerman.dw.com/en/wem-geh%C3%B6rt-das/l-37372077/lv p out
    Word: der Dialog, die Dialoge
    Saved file: der Dialog, die Dialoge
    Word: der Nachmittag, die Nachmittage
    Saved file: der Nachmittag, die Nachmittage
    Word: der Satz, die Sätze
    Saved file: der Satz, die Sätze
    ...

Hope this script proves useful to somebody else!

Happy (German) Language Learning!

Released under MIT License
