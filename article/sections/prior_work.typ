#import "../markers.typ": refine, todo

= Prior Work <sec:prior_work>

#todo[Mention Terraria "Living Trees"? They were important inspiration after all...]

#todo[
  Mention Minecraft mods that were used in thesis slides?
  Sounds fair to mention, considering that we can look at their code and see how large trees/structures were generated in practice. This gives us a good glimpse into upsides and pitfalls.
]

== Growing Neural Gas
#refine[
  Supervisor suggested checking out @GNG networks @growing_neural_gas when the initial goal was established.
  Even after exploring various @GNG implementations, we concluded that @GNG was not particularly relevant to the subject of growing trees, nor was it effective for real-time even with major optimizations @growing_neural_gas_efficient.

  Possible uses we came up with were related to creating bounding geometry for tree crowns. These could be used to spawn @SC attractors (more on that later), or to construct meshes for rendering (which grew out of scope).

  On the topic of @GNG, we also explored the @LOD aspect and found interesting applications for compressing point cloud colors @neural_gas_color_object_reconstruct, but without relevance to _growing_ interesting trees.
]

== Space Colonization
#todo[
  - Algo and how it's good for trees @trees_with_spa_col

  - Shadow propagation based approach mentioned in @self_organizing_tree_synthesis

  - How environment/weather/light can be used to affect @SC @windy_tree_stress_response
]

#todo[
  - mention R\* tree @rstar_tree (found by examining @dbscan_clustering) as critical optimization over other data-structures

  - evaluate performance claims in @ckd_tree
]

== Neural Cellular Automata

#todo[Explain the algo and how it may be an interesting avenue for vegetation @growing_neural_cellular_automata]

#todo[Mention other attempts and uses outside of trees @growing_3d_artefacts @learning_generate_3d_shapes]

== Environmental Effects
#todo[Explain L-systems mentioned in @compelling_procedural_3d_env_landscapes]

// TODO: find more papers?!

#todo[Explain realistic tree models found in @procedural_diverse_trees and @self_organizing_tree_synthesis]

#todo[Explain how environment/weather/light affects tree growth @windy_tree_stress_response]
