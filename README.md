# fileRenamer
Adds leading 0's to every file in specified directory

Why: if first file is 1.jpg and every file increments until 100.jpg, some devices sort the files like 1.jpg, 10.jpg, 100.jpg, 101.jpg as they sort by the first number.

In order to have file sorted correctly you can add leading 0's to the file names so it will sort the files like 001.jpg, 002.jpg, 003.jpg 

(number of leading 0's depend on the amount of characters in the longest file name)

This program takes in the path to the directory and adds 0's to every file in the directory. (don't include quotes in the input)

Todo:
- optimizations could probably be done
- better error handling
- more file renaming tools
