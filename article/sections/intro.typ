#import "../markers.typ": todo, refine

= Introduction <sec:intro>

#todo[
  - Modeling of tree growth is useful for xyz...

  - describing a volume of a structure or grid.

  - these are advanced methods; mainly @NCA

  - sense-making: hot for modern systems and black-boxes
]

// TODO: is Scope a better section name?
== Motivation <sec:intro_motivation>

#refine[
  The first concept for this thesis went along the lines of "procedural noise-based terrain with @LOD", 
  stemming from the lack of extreme view distances in real-time procedurally generated game worlds [src?]. 
  Improving on this feature gap turned out vague even with basis in prior art [link No Man's Sky, Minecraft, etc.]. 
  Questions arose; Why is @LOD not mainstream? 
  Is there player interest in increased view distance? 
  This doubt lead to an early narrowing in scope towards "procedural trees with @LOD". 
  
  Vegetation can make for effective decoration in making terrain varied and more interesting to players [src?]. 
  Trees in particular play an important role due to their relative size in the player's field of view, acting as subliminal landmarks @perceiving_realism_of_procedural_trees_in_games.
  We may exploit this through distinct mutations, 
  with one of the most obvious variations being tree height when accounting for distance [src ...me? find science?]. 
]

#todo[
  4 Questions (1,2,3 can essentially be answered with personal experience):
    1. problem - 
    2. what others have done - billboards bad, 
    3. why the problem persists / ongoing research
    4. what our research adds to this area (algos?)
]

#todo[review on LOD: https://ieeexplore.ieee.org/abstract/document/1323963]

== Problem <sec:intro_problem>

When generating procedural worlds for games, the designer tends to distribute resources and skills in a way that is most noticeable to the player #todo[src?]. 
This tendency has a decisive effect in the context of visual features; nearby objects get more attention from the player and in turn the designer, while distant objects lose importance.
In linear or closed-world games, the designer can deliberately choose where to allocate resources since the playable area is limited #todo[src?]. 
Procedural generation greatly complicates such decisions, as we can no longer focus on specific parts, but have to instead produce a coherent whole. 
To cope with the vast search space, the designer intentionally imposes rules and constraints. 


== Existing Solutions <sec:intro_solutions>

Since the player viewport into the virtual world is limited, we can exploit various tricks to lighten the burden on both developers and hardware.


== Ongoing Research <sec:intro_research>

There is no solid example of a commercially successful voxel-based game as of yet that showcases extensive use of @LOD for terrain in a way that meaningfully integrates with gameplay #todo[src?]. 

As a closely related genre, space exploration games like 
_Space Engineers_ and _No Man's Sky_ 
provide long view distance almost out of necessity. 
Striving for realistic scales without any sort of @LOD is technically infeasible #todo[src...?].
Without a preview or map of sorts #todo[mention Twilight Forest map], 
the player would have a difficult time navigating planets, 
let alone choosing where to land their ship #todo[src?].
Both games use hierarchies of voxels to store and represent terrain, 
albeit visualize them with _marching cubes_ for a smooth look @marching_cubes, 
compared to the blocky look people may be used to when they think of voxels #todo[src that thought?].

Getting back to the _Minecraft_ scene, many recent technical achievements regarding @LOD are modifications (mods) to the base game by passionate developers in the community.
Working with a large existing codebase is difficult, and this is reflected in the way @LOD is approached; 
popular mods, such as 
#link("https://gitlab.com/distant-horizons-team/distant-horizons", [_Distant Horizons_]) and 
#link("https://github.com/MCRcortex/voxy", [_Voxy_]) 
generate simplified geometry from the ground truth, 
which can be framed as a mix of view-dependent and hierarchical @LOD.
The obvious drawback is how expensive and wasteful this process can be, generating fully detailed chunks when only a few upper sections happen to be visible.

#link("https://github.com/xandergos/terrain-diffusion-mc")
#todo[mention MC terrain diffusion mod?]

#todo[for @sec:discussion: theory on how noise-based terrain could be inherently generated at different scales]

#todo[mention MC sections and @LOD mods]

#todo[mention https://veloren.net/, apparently]

#todo[mention ray/path-tracing and BVH acceleration? especially Nanite from UE5, and voxel-Lumen for foliage]


== Survey <sec:intro_survey>

Given the state of things, we can finally revisit our problem; 
why do procedurally generated games struggle to present the vastness of their worlds?
This turns out to be a vast question in its own right #todo[intertwine or merge with @sec:intro_motivation?].
A simple theory is that long view distances need too large of a technical investment for an unproven feature #todo[sources?!].
