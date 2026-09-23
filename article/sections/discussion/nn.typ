#import "../../markers.typ": todo

== Neural Networks <sec:discussion_nn>

Machine learning was a recurring topic during our search,
and the pool of papers was difficult to filter from the frequent occurence.
Perhaps unsurprising was the interplay between point clouds of terrain 
and vision systems for self-driving cars, but we digress.
Since our priorities center around real-time generation,
we tried to look away from prominent solutions such as transformers or diffusion.
These techniques of today, even considering modern hardware, 
leave a lot to be desired across the board when it comes to 
performance, efficiency, efficacy, and training #todo[src?].
The controllability from a designer perspective is also poor.

This did not stop others from attempting to replace noise-based terrain 
with something more aesthetic and novel
#footnote[https://github.com/xandergos/terrain-diffusion-mc].
