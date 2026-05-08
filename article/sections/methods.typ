#import "../markers.typ": todo

= Methods <sec:methods>

== Rendering Framework
To ease experiments regarding our algorithms of choice, we develop a minimal framework with the primary purpose of visualizing point clouds, since most of the data we are working with is effectively represented by points.

The choice of tech stack was driven by the performance-oriented nature of this research, which is why we settled on Rust early and out of familiarity. This gave us access to high-quality libraries that provide essentials like graphics @API:s (`wgpu`), immediate mode @GUI:s (`egui`), and acceleration structures (`rstar`).

== @SC:lo

We implemented the @SC algorithm from the ground up by following the method section for tree modeling @trees_with_spa_col. 
@NN search is a fundamental part of the algorithm, and a $O(n^2)$ loop over the set of attractors is untenable. Thus an octree library (`oktree`) was used as the first acceleration structure, getting us close to real-time generation. 

Octrees served us well throughout development, but it became apparent that iterating over many large overlapping spheres was expensive when influence radii $d_i$ on attractors was large.
A naive workaround was to constrain the search to a certain radius #todo[(src: found under a Unity impl of @SC on GitHub)], but this deviates us from the algorithm, in turn worsening growth dynamics by disregarding distant attractors.

Getting reminded that we are working with volumes and not just points, we pursued a spatial hash with cells that each referenced all intersecting attractors. These amortized lookups gave us a decent speedup, but increased both memory usage and removal time due to excessive duplication. Cost remained high even while utilizing indirect references (common in Rust to sidestep lifetime issues [src?]) as small as 16-bit; enough for $2^16-1=65535$ attractors. 

The structures we explored usually trade construction time and memory for improved lookup times. This was actually the reason for us looking beyond Voronoi diagrams in the first place, specifically with the expensive Delaunay triangulation step needed to construct them. 
An aspect we paid less attention to but is worth noting is tree balance and quality, which can vary significantly as the attractors are killed. 

So far, neither octrees or spatial hashing improved the original slowdown caused by smaller segment size $D$. 
In fairness, this will technically always be the case; smaller segments lead to fewer attractors potentially being killed per new node, increasing the amount of iterations needed to consume all reachable attractors. 

At some point, our supervisor seemed to recognize this problem as related to @DBSCAN:s @dbscan_clustering, which led us to our final structure of choice: the R\*-tree. Using an existing library (`rstar`), we managed to further decrease lookup time, trivialize construction cost while creating an optimal tree since we have all points upfront, all while maintaining fast @NN search regardless of influence radius. This is now a functionally faithful reimplementation of the @SC algorithm.

#linebreak()

#todo[Using @SDF to spawn attractors]

== @NCA:lo
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
