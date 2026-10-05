== @SC:lo
Our @SC implementation as described in @sec:artifact_sc. 
The figures below showcase a simple render of 
input attractor clouds and output trees.
The box is filled by 20000 points,
randomly distributed across $200 times 225 times 200$ units.
The initial node is spawned $75$ units below the center.

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

#line(stroke: 0pt)

// TODO: Improve image readability. Add more in appendix.

#grid(
  columns: 2,
  row-gutter: 1em,
  [#sc_img(
    "../../img/SC,before,box.png",
    [@SC box before growth.],
    fill: rgb("#646464"),
  )],
  [#sc_img(
    "../../img/SC,after,box.png",
    [@SC box after growth.],
  )],

  [#sc_img(
    "../../img/SC,before,egg.png",
    [@SC egg before growth.],
    fill: rgb("#646464"),
  )],
  [#sc_img(
    "../../img/SC,after,egg.png",
    [@SC egg after growth.],
  )],
)

/*
  TODO:
  - demonstrate parametrized examples from exploration

  // TODO: maybe this goes into discussion too?
  - variation comes from a combination of point cloud shape, influence distance, kill distance.
*/

// TODO: colored table.cell based on importance
#let ms(value) = [#value;ms]

#figure(
  [
    // TODO: sift through git and get *exact* timings for buckets and octree, 
    //       and maybe be more concise about the Memory column...

    #table(
      columns: 4,
      [], [Construct], [Lookup], [Memory],

      // TODO: load table data from file?
      [Octree], [#ms(100)], [#ms(1500)], [High],
      [Spatial Hash], [#ms(200)], [#ms(800)], [Medium],
      [R\*-tree], [#ms(1.5)], [#ms(300)], [Low],
    )],
  caption: "Data structure metrics for 20000 randomly distributed attractors.",
) <data_structure_metrics>