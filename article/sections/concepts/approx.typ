At the beginning of 3D graphics, avoiding unnecessary work was crucial to render scenes in real-time. 
Rasterization was the core bottleneck, so painting only visible parts of polygons front-to-back could skip a lot of processing @front_to_back_bsp_tree. 
That bottleneck has since shifted wildly, focusing the solution space around software design rather than hardware limitations, 
and the spirit of avoiding work lives on by embracing approximations.
@LOD refers to the complexity of a 3D model, and is commonly known by the act of reducing detail based on distance @review_on_lod.
Triangle meshes can be simplified by removing vertices that contribute little based on perspective, 
most often baked algorithmically into a couple variants as part of an offline content pipeline. 
At the extreme end of @LOD, a 3D model is pre-rendered into a 2D sprite, known as a billboard. 
