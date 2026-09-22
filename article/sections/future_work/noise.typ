#import "../../markers.typ": todo, refine

== Scalable Noise <sec:future_noise>

#todo[
  expand on @sec:intro and @sec:intro_solutions; "generating noise at different @LOD":
]

#todo[_Distant Horizons_ recently gained support for generating a rough heightmap of "the entire world".]

#todo[mention https://veloren.net/, apparently. looks like nice heightmaps]

#refine[
  _Future:_ Investigate how much effort it could be to design a node-based terrain generation system (akin to @MC density functions usable from datapacks) from the ground up that incorporates @LOD features from the very beginning. 
  Existing systems such as @MC are difficult to rewrite in a way that's backwards-compatible with an @LOD paradigm.
  The @MC system is just not built for scale, which really showed when the world height was increased in the Caves and Cliffs updates.
  A freshly designed system should be written with parallelism and determinism in mind, to avoid the painful pitfalls of @MC.
  It's not clear how to handle features/structures in a good way, which is relevant even for small trees. 
  Anything that can cross chunk boundaries usually needs special care (#todo[mention how Hytale, which is built fresh, hides most chunk-boundary details 
  away from the designer]).
]

#todo[mention @MC terrain diffusion again because it's quite the achievement (first mentioned in @sec:discussion_nn)]
