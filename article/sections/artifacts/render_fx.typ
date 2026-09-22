#import "../../markers.typ": todo

== Rendering Framework

To ease experiments regarding our algorithms of choice, 
we developed a minimal framework with the primary purpose of 
visualizing point clouds, since most of the data we are working with 
is effectively represented by points.

The choice of tech stack was driven by the performance-oriented nature of 
this research, which is why we settled on Rust early and out of familiarity. 
This gave us access to high-quality libraries that provide essentials like 
graphics @API:s (`wgpu`), immediate mode @GUI:s (`egui`), 
and acceleration structures (`rstar`).

#todo[Write more?]