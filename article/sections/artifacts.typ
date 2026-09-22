#import "../markers.typ": todo, refine

= Artifacts <sec:artifacts>

== Rendering Framework
To ease experiments regarding our algorithms of choice, 
we developed a minimal framework with the primary purpose of 
visualizing point clouds, since most of the data we are working with 
is effectively represented by points.

The choice of tech stack was driven by the performance-oriented nature of 
this research, which is why we settled on Rust early and out of familiarity. 
This gave us access to high-quality libraries that provide essentials like 
graphics @API:s (`wgpu`), immediate mode @GUI:s (`egui`), 
and acceleration structures (`rstar`).

#todo[Write more?]

== @SC:lo <sec:artifact_sc>
We implemented the @SC algorithm from the ground up by following 
the method section for tree modeling @trees_with_spa_col. 
@NNS is a fundamental part of the algorithm, 
and a $O(n^2)$ loop over all attractors is untenable. 

=== Data Structures
Thus an octree library (`oktree`) was used as the first acceleration structure,
getting us close to real-time generation. 
Octrees served us well throughout development, but it became apparent that 
iterating over many large overlapping spheres was expensive when 
influence radii $d_i$ on attractors were large.
Naively constraining the search to a certain radius would deviate us from 
the algorithm, in turn worsening growth dynamics by disregarding distant attractors.

Realising that we worked with volumes and not just points, 
we pursued a spatial hash with cells that each referenced 
all intersecting attractors. 
These amortized lookups gave a decent speedup, 
but increased both memory usage and removal time due to excessive duplication. 
Cost remained high even while utilizing packed references as small as 16-bit; 
enough for $65535$ attractors, or $2^16$ indices minus $1$ tombstone. 
Generally referred to as a _memory arena_, it allows self-referential indices 
and reduces data movement #todo[src?].

The structures we explored usually trade construction time and memory for 
improved lookup times. 
This was the reason for us looking beyond Voronoi diagrams in the first place,
specifically with the expensive Delaunay triangulation needed to construct them. 
An aspect we paid less attention to, but is worth noting, is 
tree balance and quality, which can vary at runtime as 
attractors are killed and nodes are spawned. 

So far, neither octrees or spatial hashing proved effective against 
slowdown caused by smaller segment size $D$. 
Our supervisor seemed to recognize this problem as related to _DBSCAN_
@dbscan_clustering, leading to our final structure of choice; the R\*-tree. 
Using an existing library (`rstar`), we managed to further decrease 
lookup time and trivialize construction cost while creating an optimal tree 
since we have all points upfront, all while maintaining fast @NNS 
regardless of influence radius. 
This was now a faithful reimplementation of the @SC algorithm.

=== Parameters
#todo[explain how parameters can be expanded upon, e.g. alternating node activations, non-immediate attractor death]

#todo[these params are related to but quite far away from a more "polished product" like presented in @procedural_diverse_trees section 4.1 appendix A]

The main variables $d_i$, $d_k$, and $D$ can lead to very different 
generation times for a particular attractor cloud. 
Smaller segments lead to fewer attractors potentially being killed per new node,
increasing the amount of iterations needed to consume all reachable attractors. 
Our choice of data structures may greatly alleviate the cost of 
large influence distances, but this variability can fundamentally 
not be eliminated without a different growth strategy and 
would likely require a new approach overall. 

=== Attractor Spawning
#todo[describe how the cloud shapes the tree]

#todo[Using @SDF to spawn attractors]

=== Versatility
With our definition of real-time #todo[define real-time in intro maybe?], 
we luckily do not need to worry about achieving predictable time complexities. 
We can tune both parameters and data, allowing designers to also 
allocate time towards post-processing of the generated skeleton. 
Point clouds are unlikely to be useful in their raw form, 
#todo[incorporate @procedural_diverse_trees ref] but the methods for sprucing them up are vast. 

The @SC paper already mentions decimation and subdivision along branch curves, 
but these operate on points to improve detail. 
An example that is more relevant to us, and closer to the tail end of 
the content pipeline, is voxelization; the act of turning smooth geometry 
into a rigid grid #todo[any good reference?]. 
#todo[shortly describe point->voxel process (and e.g. ->MC block)]

=== Incremental Growth
#todo[@SC can be grown over multiple steps (or over multiple game frames) allowing for massive structures without a hitch]



== @NCA:lo
#refine[
  Describe the experiment around @NCA @growing_3d_artefacts: 
  - Reproducing the open-source build found on GitHub by updating Python packages. 
  - Exploring the provided Jupyter notebook.

  - Modifying the loss function to achieve different growth patterns. The original loss function is a combination of _Softmax_ and _negative log likelihood_ loss #footnote[https://docs.pytorch.org/docs/stable/nn].

]

#refine[
  - This algo is a black-box, and may be related to evolutionary algos.
  - This algo struggles to settle on novel shapes, i.e. anything outside training data.
]

#todo[
  - Pruning methods; why is it important, how to measure?

  - How easy is it to control appearance?

  - Run a test? Benchmark?

  - Insights around @NCA loss function (helps reach a goal), not particulary a ready product.
]