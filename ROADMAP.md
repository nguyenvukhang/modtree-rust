# Roadmap

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
