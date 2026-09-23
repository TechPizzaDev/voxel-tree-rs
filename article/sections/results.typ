#import "../markers.typ": refine, todo

= Results <sec:results>

== @SC:lo
#refine[@SC reimplementation:
  - We tested spatial hash buckets, and our attempt suffered from data duplication and high overlap during lookups.

  - We also tested octrees, which had the highest construction time, unbalanced storage, and unbalanced lookup time.

  - Insertion parameters for our R\* tree can be tuned, but only for marginal gains.
]

#let sc_img(source, caption, fill: none) = figure(
  rect(
    box(image(source), height: 128pt, clip: true, inset: (
      bottom: -108pt,
    )),
    fill: fill,
    stroke: 0.5pt + gray,
    inset: 0pt,
  ),
  caption: caption,
)

#linebreak()
#todo[
  Improve image readability. Add more in appendix.
]

#grid(
  columns: 2,
  row-gutter: 1em,
  [#sc_img(
    "../img/SC,before,box.png",
    [@SC box before growth.],
    fill: rgb("#646464"),
  )],
  [#sc_img(
    "../img/SC,after,box.png",
    [@SC box after growth.],
  )],

  [#sc_img(
    "../img/SC,before,egg.png",
    [@SC egg before growth.],
    fill: rgb("#646464"),
  )],
  [#sc_img(
    "../img/SC,after,egg.png",
    [@SC egg after growth.],
  )],
)

#todo[
  - demonstrate parametrized examples from exploration

  // TODO: maybe this goes into discussion too?
  - variation comes from a combination of point cloud shape, influence distance, kill distance.
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
      [Spatial Hash], [#ms(200)], [#ms(800)], [High],
      [R\*-tree], [#ms(1.5)], [#ms(300)], [Low],
    )],
  caption: "Data structure metrics for 20000 randomly distributed attractors.",
) <data_structure_metrics>

Hardware:
- CPU: AMD Ryzen 7 7700X 8-Core
- RAM: $2 times 32$ GB, DDR5 5200 MT/s, CL32

== @NCA:lo
#refine[Preliminary result around @NCA @growing_3d_artefacts:
  - Growth quickly collapses by scaling weights with values less than one.
  - Introducing more randomness to cell propagation creates spurious growths, which may be effective for tree variation, but may appear too close to unnatural overgrowth without introducing new restrictions.

  #grid(
    columns: 2,
    gutter: 2pt,
    [#figure(
      image("../img/NCA,original.png"),
      caption: [Original @NCA tree],
    )],
    [#figure(
      image("../img/NCA,modified,lifemask_0.2,fire_0.75.png"),
      caption: [Experimental @NCA tree with factors: $"mask"=0.2, "fire"=0.75$],
    )],
  )
]
