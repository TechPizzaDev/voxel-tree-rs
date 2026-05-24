#import "../markers.typ": refine, todo

= Algorithms <sec:algos>

Methods listed in this section follow the idea outlined by @sec:concept_approx.

#todo[
  intro paragraph:
  - real-time generation for games
]

== @GNG:lo
#refine[
  Supervisor suggested checking out @GNG networks @growing_neural_gas when the initial goal was established.
  Even after exploring various @GNG implementations, we concluded that @GNG was not particularly relevant to the subject of growing trees, nor was it effective for real-time even with major optimizations @growing_neural_gas_efficient.

  Possible uses we came up with were related to creating bounding geometry for tree crowns. These could be used to spawn @SC attractors (more on that later), or to construct meshes for rendering (which grew out of scope).

  On the topic of @GNG, we also explored the @LOD aspect and found interesting applications for compressing point cloud colors @neural_gas_color_object_reconstruct, but without relevance to _growing_ interesting trees.
]

== @SC:lo <sec:algos_sc>
#todo[
  we can classify the methods, starting with @SC:
  this is a space-filling algo, what else exists?
]

#todo[
  The authors of @trees_with_spa_col are proud that their solution "grows like actual trees". Ties to @sec:concept_approx.
]

#todo[
  - Algo and how it's good for trees @trees_with_spa_col

  - Shadow propagation based approach mentioned in @self_organizing_tree_synthesis

  - How environment/weather/light can be used to affect @SC @windy_tree_stress_response
]

#todo[
  - mention R\* tree @rstar_tree (found by examining @dbscan_clustering) as critical optimization over other data-structures

  - evaluate performance claims in @ckd_tree
]

== @NCA:lo

#todo[Explain the algo and how it may be an interesting avenue for vegetation @growing_neural_cellular_automata]

#todo[Mention other attempts and uses outside of trees @growing_3d_artefacts @learning_generate_3d_shapes]
