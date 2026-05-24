#import "../markers.typ": todo, refine

= Introduction <sec:intro>

#todo[
  - these are advanced methods; mainly @NCA

  - sense-making: hot for modern systems and black-boxes
]

Procedurally generated content has seen decent traction in games over the years. 
Techniques come in numerous shapes and sizes,
however the _survival sandbox_ genre sticks out as particularly demanding across many aspects #todo[src?].

The initial concept for this thesis went along the lines of "procedural noise-based terrain at massive scale", 
stemming from the lack of real-time procedural game worlds that utilize their endless generators for extreme view distances #todo[src?]. 
Improving on this feature gap given our limited time turned out vast and vexing, even with basis in prior art #todo[link _No Man's Sky_, _Minecraft_, etc.]. 
This doubt lead to an early narrowing in scope towards "procedural trees", pivoting us into familiar yet very relevant territory.      

Vegetation can make for effective decoration after all, 
often carrying the visuals by fleshing the world with familiar colors and improving immersion for players #todo[src?]. 
Trees in particular play an important role due to their relative size in the player's field of view, acting as subliminal landmarks @perceiving_realism_of_procedural_trees_in_games.
We briefly continue this topic of appeal under @sec:future_barcode. 

== Problem <sec:intro_problem>




== Existing Solutions <sec:intro_solutions>

Our research puts technical aspects on the spotlight, 
but games are undeniably more than just engineering problems to be solved in a vacuum.
Thus we leave non-technical solutions and topics up for discussion under @sec:discussion.

Since the player viewport into the virtual world is limited, 
we can exploit various tricks to lighten the burden on both developers and hardware.
These techniques usually fall under the term @LOD; see @sec:concept_lod for more. 


#todo[billboards bad]



== Ongoing Research <sec:intro_research>

There is as of yet no solid example of a commercial game that  successfully showcases extreme view distance in a way that meaningfully integrates with gameplay #todo[src?]. 
This means less representation of procedural features.

Under the space-themed part of our genre, exploration games like 
_Space Engineers_ and _No Man's Sky_ 
provide long view distance almost out of necessity. 
Striving for realistic scales of the universe without any sort of @LOD is technically infeasible #todo[src...?].
Without a preview or map of sorts #todo[mention Twilight Forest map], 
the player would have a difficult time navigating planets, 
let alone choosing where to land their ship #todo[src?].
Both games use hierarchies of voxels to store and represent terrain, 
albeit visualize them with _marching cubes_ for a smooth look @marching_cubes, 
compared to the blocky look people may be used to when they think of voxels #todo[src that thought?].

Around the _Minecraft_ scene, recent technical achievements regarding @LOD are modifications (mods) to the base game,    made by passionate developers in the community.
Working with a large existing codebase is difficult, and this is reflected in the way @LOD is approached; 
popular mods, such as 
#link("https://gitlab.com/distant-horizons-team/distant-horizons", [_Distant Horizons_]) and 
#link("https://github.com/MCRcortex/voxy", [_Voxy_]) 
generate simplified geometry from the ground truth, 
which can be framed as a mix of view-dependent and hierarchical @LOD.
The obvious drawback is how expensive and wasteful this process can be, generating fully detailed chunks when only a few upper chunk sections happen to be visible.

#link("https://github.com/xandergos/terrain-diffusion-mc")
#todo[mention MC terrain diffusion mod?]

#todo[for @sec:discussion: theory on how noise-based terrain could be inherently generated at different scales]

#todo[mention MC sections and @LOD mods]

#todo[mention https://veloren.net/, apparently]

#todo[mention ray/path-tracing and BVH acceleration? especially Nanite from UE5, and voxel-Lumen for foliage]


== Survey <sec:intro_survey>

Given the state of things, we can finally revisit our problem; 
why do procedurally generated games struggle to present the vastness of their worlds?
This turns out to be a vast question in its own right #todo[intertwine or merge with intro motivation?].
A simple theory is that long view distances need too large of a technical investment for an unproven feature #todo[sources?!].

#todo[what our research adds to this area (algos?)]