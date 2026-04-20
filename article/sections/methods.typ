#import "../markers.typ": todo

= Methods <sec:methods>

== Space Colonization
#todo[Describe our Rust reimplementation of @SC @trees_with_spa_col.
  - Using @SDF to spawn attractors
  - Weave prior work into results regarding various data structures for nearest-neighbor search optimization.
  - Original paper used Voronoi diagram (computed using 3D Delaunay triangulation).
]

== Neural Cellular Automata
#todo[
  Describe the experiment around @NCA @growing_3d_artefacts: 
  - Reproducing the open-source build found on GitHub by updating Python packages. 
  - Exploring the provided Jupyter notebook.

  - Modifying the loss function to achieve different growth patterns. The original loss function is a combination of _Softmax_ and _negative log likelihood_ loss #footnote[https://docs.pytorch.org/docs/stable/nn].

]

#todo[
  - pruning methods; why is it important, how to measure?
  - how easy is it to control appearance?

  - run a test? benchmark?

  - would like to see: insights around @NCA loss function (helps reach a goal), not particulary a ready product
]

== Persistent Homology
#todo[Measure/identify tree shape with "barcodes" @persistent_homology]
