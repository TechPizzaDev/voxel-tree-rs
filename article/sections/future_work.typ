#import "../markers.typ": todo

= Future Work <sec:future_work>

== Scalable Noise // TODO: move to bottom?
#todo[
  expand on @sec:intro_solutions; "generating noise at different @LOD"
]

== @LOD:lo
#todo[
  few insights into potential level-of-detail support (which was the initial motivation/goal).

  _Future:_ properly investigate the viability of these algorithms in a real-time environment, with the crux being that it should be scalable for large procedural worlds.
]

\

#todo[minimal rendering framework; the trees are not interesting/novel compared to prior work, and visualization of experiments was hindered by the framework.

  // TODO: there are still 6 weeks to improve rendering of our spacol client
  _Future:_ better rendering to
  1. spend less time in debugging/testing
  2. attempt reaching parity with @self_organizing_tree_synthesis in particular
]

== Persistent Homology <sec:future_barcode>
#todo[Measure/identify tree shape with "barcodes" @persistent_homology]

#todo[
  https://github.com/peterbraden/genetic-lisa
  - related to @persistent_homology and 
  - abstract triangles vs rigid cells (of @NCA)
  - genetic/neural algo to spawn @SC attractors?
]

#todo[
  We may exploit this through distinct mutations, 
  with one of the most obvious variations being tree height when accounting for distance #todo[src ...me? find science?]. 
]

#todo[
  tie back with @perceiving_realism_of_procedural_trees_in_games
]