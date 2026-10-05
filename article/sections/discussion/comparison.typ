== Comparison

=== Performance

==== @SC
It is difficult to make a performance comparison with the original
Voronoi diagram solution in @trees_with_spa_col because
they do not list concrete numbers, and we were not able to reproduce their setup.
Suffice to say, constructing a R\*-tree in bulk should be 
in the same ballpark as building a Voronoi diagram, 
as both structures self-balance in a similar manner.
We did find an optimized KD-tree construction @ckd_tree,
but the numbers in their figures 4 and 5 do not compare
against R\*-tree, only R-tree, so we decided to not test KD-tree.

This was a pain-point we solved from the future-work section of @trees_with_spa_col.
A benefit of the R\*-tree is that segment size $D$ does not
signficantly affect lookup performance.
Insertion parameters can be tuned, but only for marginal gains.

==== @NCA and @GNG
@NCA, as presented with voxels, is not suitable for real-time generation.
@GNG was not tested since it was not used.
Training is very slow for @NN techniques in general, 
and inference is inefficient on CPU:s.

=== Customizability

==== @GNG
Needs to be trained on an existing model. 
The more you train, the better the point-cloud approximation of the model. 
We did not see how this could be used for tree generation directly, 
but findings in @sec:algos_gng_spacing could be applied to 
spawn attractors (@sec:algos_sc_attr) at more optimal distances, 
since @GNG clouds can be pre-computed at design-time and evaluated cheaply later.

==== @NCA
Needs to be trained on the cells i.e. pixels or voxels of an existing shape. 
After training, it will attempt to reproduce the shape at all costs. 
Modifying the internals, like liveness mask or weights, 
is not effective in customizing the shape, and 
it is trivial to make the shape explode.

==== @SC
No training required. The designer has high control over 
the tree shape through various parameters and attractor cloud,
and it is very feasible for developers to add new behaviors and node interactions. 

Our artifact pales in comparison with mentioned papers in all aspects 
except performance. 
There was not enough time to recreate all the knobs or visuals, 
but we made sure the #box[R\*-tree] would not prohibit future expansions. 

The authors of @trees_with_spa_col were proud that their solution 
"grows like actual trees", and we agree with the sentiment. 
This also ties well into @sec:concept_approx and @sec:concept_env_effects.
