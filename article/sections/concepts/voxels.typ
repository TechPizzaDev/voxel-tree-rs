#import "../../markers.typ": todo, refine

Partioning space can help us with _separation of concerns_ /* link https://doi.org/10.1515/JISYS.2006.15.1-4.153 ?*/ in the distributed system that is our game world #todo[src?]. 
To better explain how one can utilize a hierarchy of grids, 
we can use _Minecraft_ as an example:

1. _Blocks_ are the smallest unit i.e. the voxels.
2. _Sections_ are portions containing $16^3$ blocks.
3. _Chunks_ are columns of sections -- height can vary. 
4. _Regions_ are groups of $32 times 32$ chunks.

Anything beyond blocks is practically just technical details for a player, but to us it can explain some technical intricacies.   

=== Concurrent Generation
The world is generated chunk-wise and over multiple consecutive steps, all on demand as chunks appear in the player's view distance @mcwiki.
Structure origins and biomes are calculated before blocks can be placed. 
Terrain shaping follows, and is a computationally expensive phase thanks to various density functions.
The convenient property of these formulas is their continuity, 
given that most of them are multiple octaves of gradient noise with some linear arithmetic sprinkled in between.
Density calculations are deterministic and embarrassingly parallel, in other words.

Caves and cliffs are fun and all, 
but there is a concurrency problem soon after; 
the decoration post-processing steps may place structures, vegetation, and ore -- to name a few. 
The point of congestion is a possibility of decorations intersecting multiple chunks, 
which is solved by tracking such dependencies and finalizing later.

=== Rendering
Chunks render only when fully generated, 
which is normal for chunking since it avoids the substantial overhead of maintaning rapidly shifting previews.


#refine[rendering mentioned; now transition into next section]