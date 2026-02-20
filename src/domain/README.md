# TODO: This is no longer core but rather domain/

# Core
Contains core data types and data structures essential to monitoring system information
- Core data types are defined in `primitive` modules
    - E.g., core::process::primitive::ProcessItem;
- Core data structures are defined in `model` modules
    - E.g., core::process::model::{ProcessSnapShot,ProcessSnapShotHistory};
    - `model`s are part of the application domain they do not implement UI concerns
    - Sort, selection, filter, operations in this project are largely UI concerns and are implemented in `component` modules.
    - The `model`s in core expose immutable iterators that can be used to create `UIModels/Widgets` such as `ratatui::widgets::Table`.
- Sub-modules of `core` group related `primitives` and `models`
    - E.g., the two previous examples
- The sub-module `common` is reserved for `primitives` and `models` that
are common among many sub-modules
    - E.g., core::common::bounded\_queue::BoundedQueue\<T>

TODO:
1. (completed) First implementation of core process models
2. Rebuild main event loop
3. Resolve module paths (modules have been moved around)
4. Make modules in components/ adapters/ events/ visible
