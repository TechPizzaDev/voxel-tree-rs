#import "../markers.typ": refine, todo

= Results <sec:results>

== Space Colonization
#todo[@SC reimplementation:
  - We tested custom spatial hash buckets and octrees (slowest construction, unbalanced storage, unbalanced lookup time).
  
  Elaborate on
    - default vs. tuned insertion parameters for R\* tree
    - customized spatial hashing
]

#let sc_img(source, caption) = figure(
  box(image(source), stroke: 0.5pt + gray, height: 128pt, clip: true, inset: (
    bottom: -108pt,
  )),
  caption: caption,
)

#colbreak() // TODO: remove break
#todo[
  Improve readability of these images... \
  and add more interesting images (probably as appendix)
  #grid(
    columns: 2,
    row-gutter: 1em,
    [#sc_img("../img/SC,before,box.png", [@SC box before growth.])],
    [#sc_img("../img/SC,after,box.png", [@SC box after growth.])],

    [#sc_img("../img/SC,before,egg.png", [@SC egg before growth.])],
    [#sc_img("../img/SC,after,egg.png", [@SC egg after growth.])],
  )

  #todo[
    - demonstrate parametrized examples from exploration

    // TODO: maybe this goes into discussion too?
    - variation comes from a combination of point cloud shape, influence distance, kill distance.
  ]
]

\
#todo[
  Write about data structure exploration and explain @data_structure_metrics.
]

// TODO: colored table.cell based on importance
#let ms(value) = [#value;ms]

#figure(
  [
    #todo[sift through git and get exact timings for buckets and octree, and maybe be more concise about the Memory column...]

    #table(
      columns: 4,
      [], [Construct], [Lookup], [Memory],

      // TODO: load from file?
      [Octree], [#ms(100)], [#ms(1500)], [Low],
      [Buckets], [#ms(200)], [#ms(800)], [High],
      [R\* tree], [#ms(1.5)], [#ms(300)], [Low],
    )],
  caption: "Data structure metrics for 20000 randomly distributed attractors.",
) <data_structure_metrics>

#todo[
  Hardware: 
  - CPU: AMD Ryzen 7 7700X 8-Core Processor
  - RAM: $2 times 32$ GB, 5200 MT/s, CL32
]

== Neural Cellular Automata
#refine[Preliminary result around @NCA @growing_3d_artefacts:
  - Growth quickly collapses by scaling weights with values below one.
  - Introducing more randomness to cell propagation creates spurious growths, which may be effective for tree variation, but may appear too close to unnatural overgrowth without introducing new restrictions.

  #grid(
    columns: 2,
    gutter: 2pt,
    [#figure(
      image("../img/NCA,original.png", scaling: "pixelated"),
      caption: [Original @NCA tree],
    )],
    [#figure(
      image("../img/NCA,modified,lifemask_0.2,fire_0.75.png", scaling: "pixelated"),
      caption: [Experimental @NCA tree with factors: $"mask"=0.2, "fire"=0.75$],
    )],
  )
]
