== Scalable Noise <sec:future_noise>

The initial concept and title for this research project was 
"Generating noise at different @LOD".
This can be implemented with textured heightmaps,
as is done in _Veloren_ #footnote[https://veloren.net/].
We are thinking of taking that further by generating noise at
reduced @LOD directly, skipping expensive downscaling.

=== Nodes <sec:future_noise_nodes>
We should investigate how much effort it would be to design a node-based 
terrain generation system, akin to @MC density functions usable from datapacks,
from the ground up while incorporating @LOD features from the very beginning.
Existing systems such as @MC are difficult to rewrite in a way 
that is backwards-compatible in a @LOD paradigm.
The @MC systems were simply not built for scale, which really showed 
when the world height was increased in the "Caves and Cliffs" update
and the game had to raise minimum hardware requirements.
A freshly designed system should be written with 
parallelism and determinism in mind, to avoid the painful pitfalls of @MC.

After many years in development, _Distant Horizons_ recently gained 
support for generating a rough heightmap of "the entire world", 
which transitions us nicely to the next section...

=== Terrain Diffusion <sec:future_noise_diffusion>
Picking up from @sec:discussion_nn and above,
we want to mention _Terrain Diffusion_
#footnote[https://github.com/xandergos/terrain-diffusion-mc]
for @MC because it is quite the achievement.
The backbone is the _InfiniteDiffusion_ framework developed
for this purpose, permitting cheap manipulation of unbounded tensors.

=== Boundaries
It is somewhat unclear how to handle features and structures 
under chunked grids in a good way, which is relevant even for small trees.
Anything that can cross chunk boundaries usually needs special care.
If we take _Hytale_ 
#footnote[https://hytale.com/news/2026/1/the-future-of-world-generation] 
as a freshly built example,
the engine attempts to hide most chunk-boundary details away from the designer.
This is convenient, but can prohibit design of larger structures. 
