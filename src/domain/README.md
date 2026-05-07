# Domain
Contains data types and data structures essential to monitoring system information
- Data types are defined in `primitive` modules
    - E.g., domain::process::primitive::ProcessItem;
- Data structures are defined in `model` modules
    - E.g., domain::process::model::{ProcessSnapShot,ProcessSnapShotHistory};
    - `model`s are part of the application domain they do not implement UI concerns
    - Sort, selection, filter, operations in this project are largely UI concerns and are implemented by `component` modules.
    - The `model`s in domain expose immutable iterators that can be used to create `components`.
- Sub-modules of `domain` group related `primitives` and `models`
    - E.g., the two previous examples
- The sub-module `common` is reserved for `primitives` and `models` that
are common among many sub-modules
    - E.g., core::common::bounded\_queue::BoundedQueue\<T>
