# Azymus
An experimental roguelike, written in Rust.

"Azymus" is apparently Latin for "pure" or "uncorrupted".  But I don't mean that in a puritanical or moralistic sense, or even a virtuous, paladin kind of way.  I settled on it more-or-less randomly.

This is my fifth or sixth attempt at this, after working through [the Rust + tcod tutorial](https://tomassedovic.github.io/roguelike-tutorial/).  The issues have come from Second-System'ing... my implicit ambition to make a roguelike that does everything and is the best at everything because I'm so smart, etc etc.

Specifically, the major issue was my desire to make a roguelike _simulation_ and a roguelike _game_ in the same application.  I wanted to be able to walk around and jack orcs up with a magic sword, and I also wanted to be able to make really complicated AI agents that would do neat stuff and I could just watch as they went around their digital business.

As a mediocre engineer and total beginner at both Rust and roguelikes, I found this challenging.

But staring at the word "Azymus", and thinking about this conflict between these different engineering goals of a perfect simulation and a playable game, and then things like the "Observer Effect" in both [physics](https://en.wikipedia.org/wiki/Observer_effect_(physics)) and [social sciences](https://en.wikipedia.org/wiki/Hawthorne_effect)... led me to, maybe, an interesting idea of what to explore with a roguelike.

I'm interested in the idea of the "pure," "uncorrupted," thing in this game being not a mythical fantasy land threatened by evil forces, or some kind of virginal princess, etc, but perhaps the Underdark or whatever that even a careful, thoughtful adventurer taints by blundering around like a violent idiot.

That might seem a bit too preciously postmodern or political.  I'm not really trying to draw any equivalences between one thing and any other thing, or make some kind of point, or allude to anything.  I like a little subversive deconstruction as much as the next guy, but that's not the major focus.  I'm more interested in trying to set up some good procedural generation and decent AI agents and be able to watch them do their thing.

So let's see how it goes.
