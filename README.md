# Roadmap

**Stretch goal**: Select some modules, and get all possible min-routes
(no unnecessary modules) through them, ranked by length. Filter by
particular node requirements (module, year, sem), and number of mods
per sem.

**Goal**: Select some modules, `modtree` outputs a Semester List of
which modules to take when. Hard-coded maximum of 5 mods per sem.

## Endgame I/O

Inputs

- input modules that will be done by which year, which sem.
- these are `condition` modules: won't be decided by the algo.
- input modules that want to be done by which year, which sem.
- these are `target` modules: required to be done else query fails.
- input module limit per sem.

Processing

> since everything here is planning for the future, data from the most
> current AY will be used.

- Global-flatten all `target` modules's prereqtrees.
- Combine them to one set.
- Remove the `condition` modules.
- Sort them topologically into a queue.
- Poll this queue repeatedly, filling in each sem in the plan.
- Constantly check for sem availability.
- Prioritize `target` modules.
- Every time a sem is incremented, check the `condition` modules for
  updates.

## Module completion

Pre-requisite trees are not going to be directly related to inter-node
relations.

They will only define the `Percentage of Completion` of a module.

In a user's graph, each module will be assigned a value. Call it
"Modules left to unlock." Completed modules will be assigned '0'.

As you complete modules, you will inadvertently half-complete the
pre-requisite of some modules. Call those modules `Modules Up Next`.

Conditions for a module to be Up Next:

- must have more than one pre-requisite.
- at least one pre-requisite is completed by the user.
- [FUTURE] account for semester-exclusive modules

For consistency, only modules that have pre-requsites that are
completed by the user are shown.

As you complete more modules, more Modules Up Next will show up, and
their Completion state will be shown too.
