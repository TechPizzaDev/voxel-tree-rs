#import "../../markers.typ": todo

== @NCA:lo <sec:algos_nca>
When growing this approach may look adjacent to image diffusion,
but integration of cellular automata sets them apart.
Instead of teaching large @NN:pla to refine noise in a global top-down strategy, 
we flip it bottom-up and grow the result naturally from a few seed cells.
By training a significantly smaller @NN the liveness rules,
we may replicate a given shape @growing_neural_cellular_automata.
In spirit of _Conway's Game of Life_, but with a complex black-box for rules, 
we get the aptly named @NCA.


#todo[Explain the algo and how it may be an interesting avenue for vegetation]

#todo[Mention other attempts and uses outside of trees 
@growing_3d_artefacts @learning_generate_3d_shapes]
