# KOT COMBAT GAME ENGINE
* Text based game engine developed in Rust utilizing the premise from "King of Tokyo" board game: KING OF THE TOWN. Future implementations will focus on combat, objective, and team oriented dynamics. This will be an open source project with invitation for your interations, campaigns, ideas and features to help create a better game. This will eventually become a 2.5D game as well, based on the mechanics in this game. 
## 0.0.1 ---
* Initial implementation with original lesson and design document here (), was initilaly a lesson in implementation of traits for my kids, but evolved into a lesson for myself in how best to manage variables and how to run a procedural program through Rust. 

## 0.1.1 ---
* First iteration after just demonstrating game logic and correct implementation of traits such as "Player" or "city" (eventually changed logic to make better plain English sense). End to end, without use of rerolls and without use of Energy system or cards, I have finished a working play logic and play loop demo. 

## 0.1.2 ---
* First and failed implementation of my reroll system/logic to add AFTER initial roll with each player. There are other ideas coalescing from this lesson about how possibly to tuck these functions WITHIN others, but I separated roll due to the way I store and recall die results already. In future implementations, may look into folding ALL major functions of die behind roll(). 

## 0.1.3 ---
* Successful implementation of reroll system, separated it using a parser to clear out any unknown or irregular characters (may expand to stop escape/unescape bugs) then changed my original idea on how rerolls should work. The game logic with a "3.5 " segment is still my apprehension of separating all possible functions of die into different functions. 

## 0.1.4 ---
* Successful Implementation of CARGO Package Management for easy engine revisions. 
* New Design Concept of creating a hero video game engine, first text based then eventually graphical

## 0.1.5 --- CARGO Distribution
# Learning Cargo Workspace; Notes on Future Development Philosophy:
Attempting to use this space to give insight on my thought process in creating this game to make this system easy for others to use. This engine will be open source and remain a great place for us to make a fun game. 

## For My Project: 
Utilizing the Crates focus on **global actions** and packages  **localized actions** on **die** or  *Player* **stats**. The structure proposed allows for granular tuning of things in the die system, the player logic, and overall game logic. Current game implementation propagates but does not implement energy system. This will begin the first steps of implementation of the energy expenditure/ henshin system. 

Below is proposed CARGO workspace structure to help better separate the features: 

### Cargo Workplace structure:

* src
	* main.rs // run player checks, populate names, roll dice , request rerolls. 
* core 
	* dice.rs // data definitions
	* player.rs // user (CLI/O)
	* game.rs //game logic
* pwrs 
	* traits.rs
	* combat.rs

### Learning How to Implement Cargo for this Project:
LESSONS  REFERENCE : https://www.youtube.com/watch?v=969j0qnJGi8

> Edited with [StackEdit](https://stackedit.io/).