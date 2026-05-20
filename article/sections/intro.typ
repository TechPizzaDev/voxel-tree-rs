#import "../markers.typ": todo, refine

= Introduction <sec:intro>

#todo[
  - Modeling of tree growth is useful for xyz...

  - describing a volume of a structure or grid.

  - these are advanced methods; mainly @NCA

  - sense-making: hot for modern systems and black-boxes
]

// TODO: is Scope a better section name?
== Motivation <sec:intro_motivation>

#refine[
  The first concept for this thesis went along the lines of "procedural noise-based terrain with @LOD", stemming from the lack of extreme view distances in real-time procedurally generated game worlds [src?]. 
  Improving on this feature gap turned out vague even with basis in prior art [link No Man's Sky, Minecraft, etc.]. Questions arose; Why is @LOD not mainstream? Is there player interest in increased view distance? This doubt lead to an early narrowing in scope towards "procedural trees with @LOD". 
  
  Vegetation can make for effective decoration in making terrain varied and more interesting to players [src?]. Trees in particular play an important role due to their relative size in the player's field of view, acting as subliminal landmarks @perceiving_realism_of_procedural_trees_in_games.
  We may exploit this through distinct mutations, with one of the most obvious variations being tree height when accounting for distance [src ...me? find science?]. 
]

#todo[
  4 Questions (1,2,3 can essentially be answered with personal experience):
    1. problem - 
    2. what others have done - billboards bad, 
    3. why the problem persists / ongoing research
    4. what our research adds to this area (algos?)
]

#todo[review on LOD: https://ieeexplore.ieee.org/abstract/document/1323963]

== Problem

When generating procedural worlds for games, the designer tends to distribute resources and skills in a way that is most noticeable to the player #todo[src?]. 
This tendency has a decisive effect in the context of visual features; nearby objects get more attention from the player and in turn the designer, while distant objects lose importance.
In linear or closed-world games, the designer can deliberately choose where to allocate resources since the playable area is limited #todo[src?]. 
Procedural generation greatly complicates such decisions, as we can no longer focus on specific parts, but have to instead produce a coherent whole. 
To cope with the vast search space, the designer intentionally imposes rules and constraints. 
A fundamental and common restraint is using grids to delimit space #todo[src?], which can be implicit like rooms in a dungeon, or explicit like individual voxels. 
Herein lies the tradeoff; our grid size is one of the deciding factors on how the designer spends their resources.

Take some of the possible features that can appear in an infinite sandbox game like terrain, biomes, vegetation, or structures #todo[src?] (listed roughly from large to small). 
These need to be mixed in some way to make an engaging experience, but in a procedural world, we don't know where the player will venture or the order in which they encounter features. 
We can guide them with specific cues, but it can be challenging to generate meaningful trails such as dirt paths leading to settlements, or landmarks that signal some nearby @POI #todo[src/example from cubeworld?].

Hardware limitations are a driving force as well, where grids can be particularly versatile in accommodating memory limits, but also introduce parellization opportunities.
#todo[problem section feels incomplete]

== Existing Solutions
Since our viewport into the virtual world is limited, we can exploit various tricks to lighten the burden on both developers and hardware.

== Ongoing Research


== Survey

