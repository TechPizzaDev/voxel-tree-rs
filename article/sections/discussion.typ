#import "../markers.typ": todo, refine

= Discussion <sec:discussion>

#todo[
  This involved a lot of sense-making (apparently hot topic for modern systems and black-boxes)
]

#todo[
  - Compare our performance with original @SC Voronoi structure  (slowest construction, balanced storage, balanced lookup time).

  - Compare our customizability with mentioned papers regarding tree modeling.
]

#todo[
  Describe how thesis progressed from a goal (generating trees in real-time for games) to exploring various prior work (@growing_neural_gas @trees_with_spa_col @growing_neural_cellular_automata)
]

#todo[
  - What methods can be combined, and how?
  - Loop back to @LOD and reason around feasability.
]

#todo[
  - Compare @SC vs shadow propagation @self_organizing_tree_synthesis

  - Light and shadow are key to creating natural trees. 
]

#todo[
  Discuss future work from @SC papers: convert scanned point clouds of real trees to algo params?
]

#todo[
  Explain relevance of environment in tree growth (this could go into future work...):
  - Local species (pests, competition, mycelium) may act as biomes to dictate tree attributes.

  Generating a population of trees:
  - quick poisson disc sample for initial placement?
  - elaborate on how terrain can shape placement.
]

== Games

==== Terraria
#todo[Mention Terraria "Living Trees"? They were important inspiration after all...]

==== Minecraft
#todo[
  Mention Minecraft mods that were used in thesis slides?
  Sounds fair to mention, considering that we can look at their code and see how large trees/structures were generated in practice. 
  This gives us a good glimpse into upsides and pitfalls.
]


== Neural Networks <sec:discussion_nn>
Machine learning was a recurring topic during our search, 
and the pool of papers was difficult to filter from the frequent occurence. 
Perhaps unsurprising was the interplay between point clouds of terrain and vision systems for self-driving cars, but we digress.
Since our priorities center around real-time generation, 
we tried to look away from prominent solutions such as transformers or diffusion.
These techniques of today, even considering modern hardware, leave a lot to be desired across the board when it comes to performance, efficiency, efficacy, and training #todo[src?].
The controllability from a designer perspective is also poor.

This did not stop others from attempting to replace noise-based terrain with something more aesthetic #todo[mention MC terrain diffusion mod?].
#link("https://github.com/xandergos/terrain-diffusion-mc")