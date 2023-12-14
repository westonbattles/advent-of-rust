# Day 2

Day 2 is when things started to heat up. In this problem, a game is explained in which an elf takes out an assortment of red, green and blue cubes from a bag. Each perumutation of cubes is a "round", and a game is made up of these rounds. For example the elf might take out 2 red cubes, 3 green cubes and than 1 blue cube for the first round. For the second round, the elf might take out 1 red cube and 1 blue cube.

The first problem is, given lines of these "games" ordered with an ID, you must add up the IDs of all the games possible with only 12 red cubes, 13 green cubes, and 14 blue cubes. 

For exmaple, given the following games:
```
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
```

Game 1 would be valid, as each round does not contain a value for a cube color that is higher than the provided limit, but game 3 would not be valid as it requires 20 red cubes, and we are limited to 12 red cubes.



This problem allowed me to practice idiomatic Rustacean ways to parse data from strings. I defined a ```Game``` struct, and had it represent an integer 'id', and a Vector of arrays all with length three ```game_sets```. As demonstration, the id of Game 1 in the example above would be '1', and the set of games would be a Vec<[u32; 3]>, containing ([4,0,3], [1,2,0], [0,2,0]). (The numbers in the array representing the counts of red, green and blue cubes respectively).

The idiomatic way to parse this from the game string, was to implement the ```FromStr``` trait for my ```Game``` struct. In this implementation, I defined logic to return the game and its data on success, and return a custom error type (depending on how the error was reached) on fail.

The ```FromStr``` trait allows me to call ```.parse()``` on my game string, and handle the case of a returned error. Being able to use ```.parse()``` on the string given to me from my custom line reader, due to my implementation of ```FromStr``` felt very idiomatic, and is another one of Rusts amazing strengths as a language
