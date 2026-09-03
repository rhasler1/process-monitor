# TODO:
## src/
- config ui
- config controller
- process table controller
    - Currently, reliant on key modifiers that are platform dependent,
    e.g., Alt on Mac is not friendly.
- worker threads sysinfo and event should be rewritten such that their
  respective event loops are not coupled to default.
- unit & integration tests
    - start with process table component & worker threads
    - before revising app events, look at `edit-rs` event impl
    (this supports a mock event source for testing)
- work on status messages
    - e.g., currently, user saves config and they do not receive
    feedback as to if the save was successful.
- currently, changing the refresh rate does nothing... impl this.
- capture resize events and redraw, currently, next draw doesn't occur until the next refresh event.
- fix known bug in process table widget: nothing is drawn if filter is Some but nothing matches...
    - e.g., pid = 0
- terimate & help need to be implemented

## process-table/
- for the most part this is stable
- process stats & associated columns can be expanded:
    - e.g., depending on the refresh interval and stats capacity, different time spans can be made available...
