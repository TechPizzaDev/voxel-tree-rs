#import "../../markers.typ": todo

Partioning space can help us with _separation of concerns_ /* link https://doi.org/10.1515/JISYS.2006.15.1-4.153 ?*/ in the distributed system that is our game world #todo[src?]. 
To better explain how one can utilize a hierarchy of grids, we can use _Minecraft_ as an example:

1. _Blocks_ are the smallest unit i.e. the voxels.
2. _Sections_ are portions containing $16^3$ blocks.
3. _Chunks_ are columns of sections -- height can vary. 
4. _Regions_ are groups of $32 times 32$ chunks.

World generation is done chunk-wise and over multiple consecutive steps, all on demand as chunks appear in the player's view distance @mcwiki.
Structure origins and biomes are calculated before any blocks can be placed. 
Terrain shaping follows, and is the most computationally expensive phase thanks to various density functions.
The convenient property of these formulas is their continuity, 
given that most of them are multiple octaves of gradient noise with some linear arithmetic sprinkled in between.
Density is deterministic and embarrassingly parallel, in other words.

Caves and cliffs are fun and all, 
but we have a concurrency problem right after, 
referred to as decoration; 
the various post-processing steps which place 
structures, vegetation, and ore -- to name a few. 
The point of congestion is a possibility of decoration features intersecting multiple chunks, 
requiring well-defined order and dependency tracking to resolve.

#todo[section feels incomplete]

#todo[mention rendering, and transition into next section]

