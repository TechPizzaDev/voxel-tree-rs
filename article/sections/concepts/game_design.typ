#import "../../markers.typ": todo

== Design Scope <sec:concept_design>
When crafting worlds for games, the designer tends to distribute resources 
and skills in a way that ends up noticeable to the player #todo[src?]. 
This tendency has a decisive effect in the context of visual features; 
nearby objects get more attention from the player and in turn the designer,
while distant objects lose importance.
In linear or closed-world games, the designer can deliberately choose where 
to allocate resources since playable area is limited #todo[src?]. 

=== Design Time <sec:concept_design_time>
Open-world games, especially those relying on procedural generation, 
greatly complicate resource allocation. 
We can no longer focus on discrete parts but have to instead 
produce a coherent whole. 
To cope with the vast search space, the designer may intentionally 
impose rules and constraints, forming a _design language_ with basis 
in available resources, target audience, and the artistic vision #todo[src?]. 

A design language helps from a technical standpoint too. 
Rules can guide the designer towards handcrafted and fitting content,
over spending resources on a powerful procedural system 
which carries the risk of not fitting in aesthetically.
Authored content can also be easier to optimize for runtime #todo[src?].