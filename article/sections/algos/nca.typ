#import "../../markers.typ": todo

== @NCA:lo <sec:algos_nca>

When growing this approach may look adjacent to image diffusion,
but integration of cellular automata sets them apart.
Instead of teaching large @NN:pla to refine noise 
in a global top-down strategy, 
we flip it bottom-up and grow the result naturally from a few seed cells.
By training a significantly smaller @NN on the liveness rules,
we may replicate a given shape @growing_neural_cellular_automata.

In spirit of _Conway's Game of Life_, but with a complex black-box for rules, 
we get the aptly named @NCA, which we also explore under @sec:artifact_nca.

#todo[link @growing_neural_cellular_automata in here]
@NCA generate structures through repeated local interactions between cells. Each cell maintains a state and uses a small neural network to determine how that state should change based on information from neighbouring cells. Starting from only a small number of seed cells, these local updates can gradually produce complex global structures. This makes NCA particularly interesting for procedural vegetation, since plants similarly develop through local growth processes and can potentially respond to environmental conditions. However, the limited controllability and computational requirements of NCA may make it less suitable for real-time tree generation than more explicitly controlled approaches such as Space Colonization.

#todo[Mention other attempts and uses outside of trees 
@growing_3d_artefacts @learning_generate_3d_shapes]