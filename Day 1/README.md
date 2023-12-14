# Day 1

Day 1 started off pretty simple, and proved to be extremly applicable to Rust's powerful parseing capabilities: Given each line in some input file, you must sum the number created by the first numeric and last numeric.

For exmaple if the given input file read:
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
