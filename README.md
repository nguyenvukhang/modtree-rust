# Roadmap

**Stretch goal**: Select some modules, and get all possible min-routes
(no unnecessary modules) through them, ranked by length. Filter by
particular node requirements (module, year, sem), and number of mods
per sem.

**Goal**: Select some modules, `modtree` outputs a Semester List of
which modules to take when. Hard-coded maximum of 5 mods per sem.

## Data source

Since everything here is planning for the future, data from the most
current AY will be used.

## Merging trees

Let the final target be a prereqtree itself of AND trees.

## Possible modules to take

Scan the tree for these modules:

1. no pre-requsites remaining
2. available for the current semester

If there are `n` such modules, then generate a list of `n` choose 5
modules, and branch all those into separate paths to search.

## Priority queue of state nodes

Sort options are:

1. By modules left to unlock the master tree. (But that's what we're
   trying to find)
2. By number of sems required to complete.
3. Conditionally by modules left to unlock, condition being that there
   are no duplicates remaining in the tree.
4. By number of modules left in the pre-req tree. (lower accuracy than
   naive min-path)
5. Sort by naive-best-case min-path: Length of min-path minus number
   of duplicates found in the graph.
   - this might work because dijkstra ends early when paths remaining
     are all longer, and this assures that all possible paths are
     actually scanned.

# Target I/O

Inputs

- input modules that will be done by which year, which sem.
- (hard-code) 5 module limit per sem

Pre-Processing

- create a run-specific prereqtree of the AND variant.
- put all the target modules in this tree.
- expand this tree and re-insert all modules fetched from database.
- result is one large tree where if it's fulfilled, then all the
  targets are met.

Tree Traversal

- run a Dijkstra-like search where each node is the current state of
  all the modules taken.
- All nodes represent the start/end of a sem, never half-way through
  it. At each sem, the student takes all modules possible until the
  module limit is hit.
- To get possible modules to take each sem, see the section on
  [possible modules to take](#possible-modules-to-take)
- Branch out all possibilities, increment their semesters, and push
  these into the priority queue.
