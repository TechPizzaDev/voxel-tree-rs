== @NCA:lo <sec:algos_nca>

When growing this approach may look adjacent to image diffusion,
but integration of cellular automata sets them apart.

In spirit of _Conway's Game of Life_, but with a complex black-box for rules, 
we get the aptly named @NCA, which we also explore under @sec:artifact_nca.

@NCA generates structures through repeated local interactions between cells
@growing_neural_cellular_automata. 
Instead of teaching large @NN:pla to refine noise in a top-down strategy, 
each cell maintains a state and uses a small @NN to determine how 
that state should change based on neighbouring cells. 
Starting from only a few seed cells, 
these local updates can gradually produce complex global structures. 
This makes @NCA particularly interesting for procedural vegetation, 
since plants similarly develop through local growth processes and 
can potentially respond to environmental conditions. 
However, the limited controllability and computational requirements of @NCA 
may make it less suitable for real-time tree generation than 
more explicitly controlled approaches such as @SC.
