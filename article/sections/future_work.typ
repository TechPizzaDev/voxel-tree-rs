#import "../markers.typ": todo, refine

= Future Work <sec:future_work>


== Scalable Noise <sec:future_noise>
#todo[
  expand on @sec:intro and @sec:intro_solutions; "generating noise at different @LOD":
]

#todo[_Distant Horizons_ recently gained support for generating a rough heightmap of "the entire world".]

#todo[mention https://veloren.net/, apparently. looks like nice heightmaps]

#refine[
  _Future:_ Investigate how much effort it could be to design a node-based terrain generation system (akin to MC density functions usable from datapacks) from the ground up that incorporates @LOD features from the very beginning. 
  Existing systems such as MC are difficult to rewrite in a way that's backwards-compatible with an @LOD paradigm.
  The MC system is just not built for scale, which really showed when the world height was increased in the Caves and Cliffs updates.
  A freshly designed system should be written with parallelism and determinism in mind, to avoid the painful pitfalls of MC.
  It's not clear how to handle features/structures in a good way, which is relevant even for small trees. 
  Anything that can cross chunk boundaries usually needs special care (#todo[mention how Hytale, which is built fresh, hides most chunk-boundary details away from the designer]).
]

#todo[mention MC terrain diffusion again because it's quite the achievement (first mentioned in @sec:discussion_nn)]

== Miniaturization <sec:future_mini>
#todo[
  Few insights into potential level-of-detail support (which was the initial motivation/goal).

  _Future:_ Actually investigate the viability of these algorithms in a real-time environment, with the crux being that it should be scalable for large procedural worlds.
  This thesis was not really enough time to do it on my own.
]

#todo[Apparently @NCA:s are scalable beyond the individual cells somehow? can't tell how easy that is, but seemed interesting]


== Better Rendering <sec:future_rendering>

#todo[minimal rendering framework; the trees are not interesting/novel compared to prior work, and visualization of experiments was somewhat hindered by the framework (worth it tho).

  _Future:_ better rendering to
  1. spend less time in debugging/testing
  2. attempt reaching parity with @self_organizing_tree_synthesis in particular
]


== Persistent Homology <sec:future_barcode>
#todo[Describe how tree shape can be measured or compared with "barcodes" @persistent_homology, and how this can be used by developers to guide parameters]

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

== Environmental Integration <sec:future_env>

#refine[
@sec:concept_env_effects was hot at the start, but we did not have time to experiment with the actual concepts.
The study has essentially developed one tree in a vacuum without a technical plan to integrate with neighboring vegetation and terrain.
]

One could say that attractors in @SC are themselves a form of limited resource.
