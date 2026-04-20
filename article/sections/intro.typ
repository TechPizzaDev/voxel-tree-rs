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
