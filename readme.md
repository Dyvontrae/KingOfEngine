## 0.0.1 ---
### Initial implementation with original lesson and design document here (), was initilaly a lesson in implementation of traits for my kids, but evolved into a lesson for myself in how best to manage variables and how to run a procedural program through Rust. 

## 0.1.1 ---
### First iteration after just demonstrating game logic and correct implementation of traits such as "Player" or "city" (eventually changed logic to make better plain English sense). End to end, without use of rerolls and without use of Energy system or cards, I have finished a working play logic and play loop demo. 

## 0.1.2 ---
### First and failed implementation of my reroll system/logic to add AFTER initial roll with each player. There are other ideas coalescing from this lesson about how possibly to tuck these functions WITHIN others, but I separated roll due to the way I store and recall die results already. In future implementations, may look into folding ALL major functions of die behind roll(). 

## 0.1.3 ---
### Successful implementation of reroll system, separated it using a parser to clear out any unknown or irregular characters (may expand to stop escape/unescape bugs) then changed my original idea on how rerolls should work. The game logic with a "3.5 " segment is still my apprehension of separating all possible functions of die into different functions. 

## 0.1.4 ---
### Successful Implementation of CARGO Package Management for easy engine revisions. 
### New Design Concept of creating a hero video game engine, first text based then eventually graphical