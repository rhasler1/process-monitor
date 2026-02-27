# 2/20/2026 
- Signing off here; pick back up on components/
- Implement sort & handle\_key() methods
- Improve upon view
- After the above are complete, I can look into generalizing UI Components.
    - Enum Move is a good place to start
    - General table implementation with row & col props?
        - This will be much more involved
    - Seperate table view logic from ratatui?
        - E.g., internal app table gets "plugged into" ratatui?

# Threads
- Main thread synchronizes drawing `UI Components`, `AppEvents`, and the `SysinfoWorker` (see main.rs) 
- `AppEvents` are on their own thread (see events/app\_event.rs) and communicates with the main thread via `mpsc channel`
- `SysinfoWorker` is on it's own thread (see services/sysinfo\_worker.rs) and communicates with the main thread via `mpsc channel`

# Main thread event loop control flow
1. Poll message from `SysinfoWorker` via `mpsc receiver`
    - The message is an `enumerator` that is either `Ready(DomainModel)` or `InProgress`
    - **If Ready(DomainModel):** then the main thread replaces it's current `DomainModel` with the new `DomainModel`
        - Component states are updated (enforce invariants)
    - **else:** continue
2. Draw `UI Components`
3. Poll next `AppEvent` from `AppEvents` via `mpsc receiver`
    - `RefreshEvent` => send message to `SysinfoWorker` to start creating a new `DomainModel`
    - `KeyEvent`    => modify `UI Component` state
      - `UIComponentState` enforces `DomainModel` invariant rules on state parameters
        - E.g., `UIProcessSelection` must be within the range of `DomainProcessModel`
    - `TickEvent`    => back to `1`.
