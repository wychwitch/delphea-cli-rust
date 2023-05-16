# Delphea: the decision helper

Delphea is a small tool I've created to help with my own decision paralysis when faced with a huge amount of options to rank (such as favorite books, projects I want to start next, etc). Inspired by [dragonflycave's](https://www.dragonflycave.com) [favorite pokemon picker.](https://www.dragonflycave.com/favorite.html)
## Usage
TL;DR: install delphea (right now only `cargo install delphea` is supported) and run it by typing `delphea` in your command-line!

Firstly, there are two main concepts to understand: Sheets and Entries.

A Sheet is the list for the items you would like to rank, and Entries are the items to be ranked! For example, `Pokemon` would be the sheet while `Mismagus` would be one of several entries. You can add entries by using the helpful menu, or you can use the arguments --sheet and --entry!

To make life easier, if you want to add a ton of items to a list but dont want to type out the sheet every time, you can add the name of the sheet you wish to use as default by setting the environmental variable `DELPHEA_SHEET` to the name of the sheet you want to use! Then rather than needing both --sheet and --entry arguments, you only need the --entry. 

All data is saved to a .json file in `$HOME/.local/share/delphea`.

## Included features
+ Creating, editing, and deleting of sheets
+ Creating, editing, and deleting of entries
+ Ranking of entries

## Planned features
+ Nested Sheets
+ Export Data
+ Import structured list as a sheet
