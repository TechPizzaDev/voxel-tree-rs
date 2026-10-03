== @SC:lo <sec:artifact_sc>

We implemented the @SC algorithm by following instructions in 
the tree modeling paper @trees_with_spa_col. 
@NNS is a fundamental part of the algorithm, 
and a $O(n^2)$ loop over all attractors is untenable. 

=== Data Structures and Volume
Thus an octree library (`oktree`) was used as the first acceleration structure. 
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
and can simplify deallocation by dropping whole ranges.

The structures we explored usually trade construction time and memory for 
improved lookup times. 
This was the reason for us looking beyond Voronoi diagrams in the first place,
specifically with expensive Delaunay triangulation needed to construct them. 
An aspect we paid less attention to, but is worth noting, is 
tree balance and quality, which can vary at runtime as 
attractors are killed and nodes are spawned. 

=== Speed <sec:algos_sc_speed>
So far, neither octrees or spatial hashing proved effective against 
slowdown caused by smaller segment size $D$. 
Our supervisor seemed to recognize this problem as related to _DBSCAN_
@dbscan_clustering, leading to our final structure of choice; 
the R\*-tree  @rstar_tree. 
Using an existing library (`rstar`), we managed to further decrease 
lookup time and trivialize construction cost while 
creating an optimal tree since we have all points upfront, 
all while maintaining fast @NNS regardless of influence radius. 
This was now a faithful reimplementation of the @SC algorithm.

=== Parameters
The main variables $d_i$, $d_k$, and $D$ can lead to very different 
generation times for a particular attractor cloud. 
Smaller segments lead to fewer attractors potentially being killed per new node,
increasing the amount of iterations needed to consume all reachable attractors. 
Our choice of data structures may greatly alleviate the cost of 
large influence distances, but this variability can fundamentally 
not be eliminated without a different growth strategy and 
would likely require a new approach overall. 

The main variables are far away from a "polished product" 
like presented in section 4.1 of @procedural_diverse_trees.
Even those can be further expanded upon through
e.g. alternating node activations or non-immediate attractor death.
We did not try anything fancy since this relies too much on game context.

=== Attractors <sec:algos_sc_attr>
Parameters are only half the story, 
with the placement of attractors giving a tree some distinct form.
The distance between and grouping of points determines 
where branches grow toward and consequently split.
Nodes do not actually "branch out" though --
previous nodes get assigned outstanding attractors as big groups are consumed.
Treating nodes as buds is also a clever trick, see @sec:future_env_shadow.

Future work from @SC paper(s) mention novel ideas for generating attractors.
One fun idea is to convert 3D scans of real trees to point clouds,
though the parameters are not easy to just guess from scans.
On a more technical note, we prefer sampling @SDF functions
to spawn attractors, which would be performant 
and easy for designers to pick, combine, and visualize.

=== Versatility
With our real-time requirement, 
we can worry less about theoretical time complexities. 
We can tune both parameters and data, allowing designers to also 
allocate time towards post-processing of the generated skeleton. 
Point clouds are unlikely to be useful in their raw form, 
but the methods for sprucing them up are vast @procedural_diverse_trees. 

The @SC paper already mentions decimation and subdivision along branch curves, 
but these operate on points to improve detail. 
An example that is more relevant to us, and closer to the tail end of 
the content pipeline, is voxelization; the act of turning smooth geometry 
into a rigid grid. 
A simple raycast that sets a block per step can be used to simulate subdivision.

=== Incremental Growth
@SC can be grown over multiple steps i.e. over multiple game frames or ticks, 
allowing for massive structures to appear without a hitch.
There is also the quirk of the tree growing in a natural bottom-up direction,
and if desired, a growth animation can be used instead of pop-in.
