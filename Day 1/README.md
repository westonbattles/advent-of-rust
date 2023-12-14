# Day 1

Day 1 started off pretty simple, and proved to be extremly applicable to Rust's powerful parseing capabilities: Given each line in some input file, you must sum the number created by the first numeric and last numeric.

For exmaple, if the given input file read:
```
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
```
The answer would be 12 + 38 + 15 + 77 which is 142



Doing this was extremly trivial thanks to Rust's ```char.is_numeric()``` function, and figuring out an effecient way to parse each line was the longer task.

My solution to the problem involved iterating over each character in the string, tracking if the first number had shown up yet, and tracking the current number in the iteration. If we hit a number, and first is ```False```, then we add that digit*10 to the total (as the first number is in the tens place)

After iterating, the last number would be the last current number (which we were storing), and so we just add that to the overall total. Do this for each line and gg.


### Part 2

This is where I learned that each Advent of Code problem was seperated into 2 distinct parts, requiring completion of the first in order to view the second.

In this second part, the problem revealed that some digits are actually spelled out instead of being numeric, which is a lot less trivial of a problem to tackle.


For example, an input of
```
two1nine
eightwothree
abcone2threexyz
```
should output 29 + 83 + 13 = 125

My solution involed using Rust's powerful (Hashmap)[https://doc.rust-lang.org/std/collections/struct.HashMap.html] implementation, as hashing a value results in an Option enum, with either the value or 'None'. All I had to was make a hashmap of strings for the digits 0-9, and have the hash be the actual integer value itself. Then we can window-loop through substrings of the line, hash the substring and if the value isn't None, we can add it to our variables.

My current implentation loops through substrings of sizes 3, 4 and 5 from the current character (since all the words from zero-nine either have those respective lenghts), and hashes each one, and if it exisits in the hash, it does the same logic as before with the number.
