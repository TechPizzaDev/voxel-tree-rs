#import "../markers.typ": todo, refine

= Introduction <sec:intro>

Procedurally generated content has seen decent traction in games over the years. 
Techniques come in numerous shapes and sizes,
however the _survival sandbox_ genre sticks out as particularly 
demanding across many aspects #todo[src?].

The initial concept for this thesis went along the lines of 
"procedural noise-based terrain at massive scale", 
stemming from the lack of real-time procedural game worlds that 
utilize their endless generators for extreme view distances. 
Improving this feature gap given limited time turned out vast, 
even with basis in prior art like 
_Minecraft_ #footnote[https://www.minecraft.net] or 
#box[_No Man's Sky_] #footnote[https://www.nomanssky.com]. 
This doubt lead to an early narrowing in scope towards "procedural trees", 
leading us to familiar yet relevant territory.      

Vegetation can make for effective decoration after all, 
often carrying the visuals by fleshing the world with familiar colors 
and improving immersion for players #todo[src?]. 
Trees in particular play an important role due to their relative occupancy 
of the player's field of view, to the extent of acting as 
subliminal landmarks @perceiving_realism_of_procedural_trees_in_games.
We briefly continue this under @sec:future_barcode. 


== Problem <sec:intro_problem>

With the scope in place, we need to gaze at what it takes to 
bring us up to speed for real-time rendering.
There is unfortunately some level of "chicken and egg" paradox at play here, 
where resource unavailability usually forces us into simpler 
"good enough" solutions #todo[src], see @sec:concept_design.
Trees are common and recognizable vegetation in games, 
but often rely on handcrafted, reusable, and highly stylized assets #todo[src?]. 
To address our gap in a manageable manner, we explore the 
potential of procedural generation to break visual uniformity, 
ideally without increasing development costs.

*RQ*: "Which algorithms support real-time generation of 
trees in a performant manner?"
This question was chosen with limited time and scope in mind, 
while also allowing us to explore supplementary techniques. 

== Solutions <sec:intro_solutions>

Since the player's viewport into the virtual world is limited, 
we can exploit various tricks to lighten the burden on both developers and hardware.
These techniques usually fall under the term @LOD, explained by @sec:concept_lod. 
The concept applies generally across game development, 
but is notably meaningful for trees based on their relevance in scenery, 
and high frequency as the predominant part of forests.

=== Forest
Realistic forests are notoriously expensive to develop and draw at high quality.
At the extreme end we get workarounds like _billboards_ 
(see @sec:concept_lod_billboard). 
For the ground itself, distant terrain can be estimated with heightmaps, 
but we won't dwell on terrain in this study.
Simplifying geometry is generally difficult to stylize in a 
dynamic environment without noticable shadow artifacts or pop-in, 
so many games get by with static environments. 
#todo[sources!]

=== Outer Space
So far, we only described optimized rendering of full-resolution data.
How about we change the data itself? 
Under the space-themed part of our genre, exploration games like 
_No Man's Sky_ and
_Space Engineers_ #footnote[https://www.spaceengineersgame.com/home]
provide long view distance almost out of necessity. 
Striving for realistic scales of the universe without any
sort of @LOD is technically infeasible.
These games generate data at lower resolutions from the get-go, 
which happens to be main driver behind this study.
Both games use hierarchies of voxels to store and represent terrain 
(see @sec:concept_voxels),
albeit visualize them with _marching cubes_ for a smooth look @marching_cubes, 
compared to the blocky look people may be used to when they think of voxels #todo[src that thought?].

Without a preview or map of sorts 
(#todo[mention Twilight Forest map that reveals biomes and structures]), 
the player would have a difficult time navigating planets, 
let alone choosing where to land their ship while not crashing into mountains.
Integration of long-distance sight with gameplay is a challenge. 


== Ongoing Research <sec:intro_research>

Games with space exploration at their core are a minority that 
successfully showcase extreme view distances in a way that 
meaningfully integrates with gameplay #todo[src?]. 
Games are generally the primary source of invention here, which means 
smaller representation of procedural features to borrow from.

=== Community
Around the _Minecraft_ scene, recent technical achievements regarding @LOD 
are modifications (mods) to the base game, made by the community.
Working with a proprietary codebase is difficult, and this is reflected 
in the way @LOD is approached. 
Popular mods, such as 
_Distant Horizons_ #footnote[https://gitlab.com/distant-horizons-team/distant-horizons] and 
_Voxy_ #footnote[https://github.com/MCRcortex/voxy], 
generate simplified geometry from the ground truth, 
which can be framed as a mix of view-dependent and hierarchical @LOD.
The obvious drawback is how expensive and wasteful this process can be, 
generating fully detailed chunks when only a few upper chunk sections
happen to be visible.
We steer clear of this topic to focus on feature generation instead, 
pushing theories to @sec:future_noise on how noise could inherently 
be generated at different scales.

=== Industry
And for something broader, we have Unreal Engine technologies like 
Nanite and Lumen, which have been recently improved to support foliage #todo[src].
Nanite handles highly detailed geometry, and Lumen simulates light and shadow on top.
Recent upgrades to Lumen allowed it to more efficiently deal with 
foliage by using voxelized approximations.
It ties into @sec:concept_approx, but is slightly outside our paradigm.
This is still an emerging area of research, since ray- and path-tracing 
has traditionally been, and still is, held back by hardware limitations.

== Approach <sec:intro_survey>

Given the state of things, we can revisit our overarching problem; 
why do procedurally generated games struggle to present the vastness of their worlds?
A simple theory is that long view distances need too large of a 
technical investment for an unproven feature, but a qualitative study 
would be better fit to tackle that question #todo[src?].

Our study tries to put technical aspects on the spotlight, but games are
undeniably more than just engineering problems to be solved in a vacuum.
Thus we leave many topics at just the conceptual stage, 
and non-technical solutions up to @sec:discussion.

Our approach in this study does not introduce any new algorithms to the table,
instead acting more as an opinionated guide to which methods 
are suitable over others in the context of a real-time procedural game.
