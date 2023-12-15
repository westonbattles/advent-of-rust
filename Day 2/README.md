# Day 2

Day 2 is when things started to heat up. In this problem, a game is explained in which an elf takes out an assortment of red, green and blue cubes from a bag. Each perumutation of cubes is a "round", and a game is made up of these rounds. For example the elf might take out 2 red cubes, 3 green cubes and than 1 blue cube for the first round. For the second round, the elf might take out 1 red cube and 1 blue cube.

The first problem is, given lines of these "games" ordered with an ID, you must add up the IDs of all the games possible with only 12 red cubes, 13 green cubes, and 14 blue cubes. 

For example, given the following games:
```
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
```

Game 1 would be valid, as each round does not contain a value for a cube color that is higher than the provided limit, but game 3 would not be valid as it requires 20 red cubes, and we are limited to 12 red cubes.



This problem allowed me to practice idiomatic Rustacean ways to parse data from strings. I defined a ```Game``` struct, and had it represent an integer 'id', and a Vector of arrays all with length three ```game_sets```. As demonstration, the id of Game 1 in the example above would be '1', and the set of games would be a Vec<[u32; 3]>, containing ([4,0,3], [1,2,0], [0,2,0]). (The numbers in the array representing the counts of red, green and blue cubes respectively).

The idiomatic way to parse this from the game string, is to implement the ```FromStr``` trait for my ```Game``` struct. In this implementation, I defined logic to return the game and its data on success, and return a custom error type (depending on how the error was reached) on fail.

The ```FromStr``` trait allows me to call ```.parse()``` on my game string, and handle the case of a returned error. Being able to use ```.parse()``` on the string given to me from my custom line reader, due to my implementation of ```FromStr``` felt very idiomatic, and is another one of Rust's amazing capabilities as a language.

After parsing the game data, we then define a mutable boolean variable called ```game_is_possible```, initiallized to true. we then loop through each set in a given game's sets, and then for each set, we compare each value with its corresponding value in a ```bag_limit``` constant we had defined previously, which looks like this;

``` bag_limit: [u32; 3] = [12, 13, 14] ```

Amazingly, Rust provides a very clever and concise way to compare corresponding values in 2 given iterators with the ```.zip()``` method.

```let set_is_not_valid = !set.iter().zip(bag_limit.iter()).all(|(&a, &b)| a <= b);```

This code simultianiously iterates through the first iterator (```set.iter()```), and the iterator passed as a pramater to ```.zip()``` (```bag_limit.iter()```). Calling ```.all()``` on this allows us to store reference to the two currently iterated values, and compare them. If all the comparisons are true, than the output is true. If any are false, than the output is false.

Since we have a ```!``` preceding this boolean, the output inverses and we now have a way to determine if our given set is valid or not.
If the set is not valid, than we set the variable ```game_is_possible``` to false, and we know not to add the game id to the total after looping through each set in the game.


### Part 2

Part 2 of this problem disregards the previous "limit", and instead asks you to find the minimum number of each cube requried to play each game. For example, in the above demonstration the minimum number of cubes for Game 1 would be 4 red, 6 green and 2 blue. For each line, you must take the power of the minimum cube set (which is defined as the product of the three numbers in the set). Summing up these powers than results you in the correct answer.

Due to my framework for parseing in cube games from a string, all I had to do was implement the new logic. I didn't have to spend any more time creating framework as I could still load and access data from game lines easily.

My solution for this problem involved pretty much the same logic as before, but instead of doing it with the entire sets, we have to do it with the values themselves. This means no more ```.add()``` :(

We start off by saving the first set and defining it as the ```max_set```. We then loop through the next sets in the game, and compare each individual value of the iterated set with the ```max_set```. If any of the values in the iterated set are greater, than we set the corresponding value in ```max_set``` to be that value. The code looks like this:

```
for (&val, max) in set.iter().zip(max_set.iter_mut()){
  if val > *max {
    *max = val;
  }
}
```

Since we might need to modify the actual value of the corresponding ```max_set``` value, Rust lets us use ```.iter_mut()``` instead, which lets us dereference and then mutate the value itself. So by the end of this loop, the ```max_set``` array will contain the highest three red, green and blue numbers. Beautiful right?


Last but not least, we need to sum the product of every ```max_set```, and you can do this in Rust with a single line;
```total += max_set.iter().product::<u32>(); // I love rust so fucking much```

As you can tell by my enthusiastic comment, being able to do this and cast the product into a neccesarily sized/signed integer using the  [turbofish](https://turbo.fish/) syntax, just reiterated for me how cool I already knew Rust to be.
