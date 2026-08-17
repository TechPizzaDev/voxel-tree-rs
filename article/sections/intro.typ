#import "../markers.typ": todo, refine

= Introduction <sec:intro>

Procedurally generated content has seen decent traction in games over the years. 
Techniques come in numerous shapes and sizes,
however the _survival sandbox_ genre sticks out as particularly demanding across many aspects #todo[src?].

The initial concept for this thesis went along the lines of "procedural noise-based terrain at massive scale", 
stemming from the lack of real-time procedural game worlds that utilize their endless generators for extreme view distances. 
Improving on this feature gap given our limited time turned out vast and vexing, even with basis in prior art like 
_Minecraft_ #footnote[https://www.minecraft.net] or 
#box[_No Man's Sky_] #footnote[https://www.nomanssky.com]. 
This doubt lead to an early narrowing in scope towards "procedural trees", pivoting us into familiar yet very relevant territory.      

Vegetation can make for effective decoration after all, 
often carrying the visuals by fleshing the world with familiar colors and improving immersion for players #todo[src?]. 
Trees in particular play an important role due to their relative size in the player's field of view, to the extent of acting as subliminal landmarks @perceiving_realism_of_procedural_trees_in_games.
We briefly continue this under @sec:future_barcode. 


== Problem <sec:intro_problem>

With the scope in place, we need to gaze at what it takes to bring us up to speed for real-time rendering.
There is unfortunately some level of "chicken and egg" paradox at play here, 
where resource unavailability usually forces us into simpler "good enough" solutions #todo[src], see @sec:concept_design.
Trees are common and recognizable vegetation in games, 
but often rely on handcrafted, reusable, and highly stylized assets #todo[src?]. 
To address the gap in a manageable manner, we explore the potential of procedural generation to break visual uniformity, ideally without increasing development costs.

*RQ*: Which algorithms support real-time generation of trees in a performant manner? 

#todo[mention modded minecraft / inspired by big "sacred rubber tree"? (although those all look the same)]


== Solutions <sec:intro_solutions>

Since the player viewport into the virtual world is limited, 
we can exploit various tricks to lighten the burden on both developers and hardware.
These techniques usually fall under the term @LOD, explained by @sec:concept_lod. 
The concept applies generally across game development, 
but is notably meaningful for trees based on their relevance in scenery, and high frequency as the essential part of forests.

Realistic forests are notoriously expensive to develop and draw at high quality.
This lead to workarounds like billboards, which are pre-rendered 2D sprites, usually with normal maps to help lighting. 
As for the ground itself, distant terrain can be estimated with heightmaps.
Simplifying geometry like this is usually difficult to stylize in a dynamic environment without noticable shadow artifacts or pop-in. 
#todo[sources!]

#refine[]
There is as of yet no solid example of a commercial game that  successfully showcases extreme view distance in a way that meaningfully integrates with gameplay #todo[src?]. 
Games are a primary source of invention here, which means smaller representation of procedural features to borrow from.

Under the space-themed part of our genre, exploration games like 
_No Man's Sky_ and
_Space Engineers_ #footnote[https://www.spaceengineersgame.com/home]
provide long view distance almost out of necessity. 
Striving for realistic scales of the universe without any sort of @LOD is technically infeasible.
Without a preview or map of sorts (#todo[mention Twilight Forest map that reveals biomes and structures]), 
the player would have a difficult time navigating planets, 
let alone choosing where to land their ship while not crashing into mountains.
Both games use hierarchies of voxels to store and represent terrain, 
albeit visualize them with _marching cubes_ for a smooth look @marching_cubes, 
compared to the blocky look people may be used to when they think of voxels #todo[src that thought?].


== Ongoing Research <sec:intro_research>

Around the _Minecraft_ scene, recent technical achievements regarding @LOD are modifications (mods) to the base game, made by the community.
Working with a proprietary codebase is difficult, and this is reflected in the way @LOD is approached; 
popular mods, such as 
_Distant Horizons_ #footnote[https://gitlab.com/distant-horizons-team/distant-horizons] and 
_Voxy_ #footnote[https://github.com/MCRcortex/voxy] 
generate simplified geometry from the ground truth, 
which can be framed as a mix of view-dependent and hierarchical @LOD.
The obvious drawback is how expensive and wasteful this process can be, 
generating fully detailed chunks when only a few upper chunk sections happen to be visible.
We mostly avoid this topic to focus on generation itself, 
pushing theories to @sec:future_noise on how noise could be inherently generated at different scales.

#todo[mention https://veloren.net/, apparently]

#refine[]
For something broader, we have Unreal Engine technologies like Nanite and Lumen, which have been recently improved to support foliage.
Nanite handles highly detailed geometry, and Lumen simulates light and shadow on top.
Recent upgrades to Lumen allowed it to more efficiently deal with foliage by using voxelized approximations.
This is still an emerging area of research, since ray- and path-tracing has traditionally been, and still is, held back by hardware limitations.
#refine[This ties into @sec:concept_approx, but slightly outside our paradigm.]


== Approach <sec:intro_survey>

#refine[]
Given the state of things, we can revisit our problem; 
why do procedurally generated games struggle to present the vastness of their worlds?
A simple theory is that long view distances need too large of a technical investment for an unproven feature, but a study would be better fit to tackle that question #todo[src?].

Our research puts technical aspects on the spotlight, 
but games are undeniably more than just engineering problems to be solved in a vacuum.
Thus we leave some topics at the conceptual stage, and non-technical solutions like design up to @sec:discussion.

#todo[what our research adds to this area (algos?)]

#todo[This leaves us with algorithms.]