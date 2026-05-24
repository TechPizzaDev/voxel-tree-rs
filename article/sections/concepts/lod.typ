#import "../../markers.typ": todo

At the beginning of 3D graphics, avoiding unnecessary work was crucial to render scenes in real-time. 
Rasterization was the bottleneck, so painting only visible parts of polygons front-to-back could skip a lot of processing @front_to_back_bsp_tree. 
The bottleneck has since shifted wildly, orienting the solution space around software design rather than hardware limitations, 
yet the spirit of avoiding work lives on.
This leads us to a perhaps literal interpretation of approximation --
@LOD -- referring to the complexity of a 3D model, commonly recognized by the act of reducing detail based on distance @review_on_lod.
Triangle meshes can be simplified by removing vertices that contribute little based on perspective, 
most often baked algorithmically into a couple variants as part of an offline content pipeline. 
At the extreme end, a 3D model is pre-rendered into a 2D sprite, known as a _billboard_. 