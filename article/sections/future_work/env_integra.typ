== Environmental Integration <sec:future_env>

=== Light and Shadow
Shadow propagation, as introduced by @self_organizing_tree_synthesis,
was a great nature-inspired method we liked.
It augments @SC by constraining branch splits to buds,
and by propagating a penumbra from buds, "consuming light" from buds below. 
One could say that attractors in @SC are themselves a form of limited resource,
and this optimized shadow-caster took it a step further.
From a glance we expect this to be even faster than @SC on its own,
even with the added functionality, and we would like to compare them in the future.

@sec:concept_env_effects was so hot at the start that we naively wasted time 
trying to combine shadow propagation with @NCA in the @MC environment.
@NCA was too difficult to stabilize with dynamic conditions disturbing it,
so we moved back to @SC.

=== Biomes
Our research focused on one tree in a vacuum without a direct plan forward,
instead leaving room for future integrations with neighboring vegetation and terrain.
There are _many_ variables that can affect plant growth.
Our favorite that came from a brainstorm session are localized species such as
insects and mycelium, which may act as micro-biomes that dictate tree attributes.
All these systems could interact with eachother and create interesting ecospheres.

The weather is also an important world-building tool @windy_tree_stress_response 
that extends both biome and placement aspects. 

=== Placement
We can not propose a definite optimal way to place trees,
since placement in a scene can depend on endless factors.
We recommend to keeping surrounding world and @sec:concept_approx in mind.
Try integrating with the terrain generator to avoid expensive search-based rules.
At least there is the benefit of this being mostly one-directional; 
trees are shaped by terrain, and do not usually shape terrain themselves.
