#import "../../markers.typ": todo

Partitioning space into grids can be implicit like rooms in a dungeon, or explicit like individual voxels in terrain. 
Herein lies the tradeoff; grid size becomes one of the deciding factors on how the designer spends their resources.

Take some of the possible features that can appear in an infinite sandbox game like 
terrain, biomes, vegetation, or structures #todo[src?] -- 
listed roughly from large to small. 
These need to be mixed in some way to make an engaging experience, but in a procedural world, we don't know where the player will venture or the order in which they encounter features. 
We can guide them with specific cues, 
but it can be challenging to generate meaningful trails, such as 
dirt paths leading to settlements, or landmarks that signal some nearby point of interest.

Hardware limitations are a driving force as well, where grids can be particularly versatile in accommodating memory limits, but also introduce parellization opportunities.

#todo[section feels incomplete]