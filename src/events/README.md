# Events
- Initial implementation & testing of app\_events.rs is completed.
- TODO 2/18/2026 - Read and revise

## Description
Sends events to the main event thread
- Four `AppEvent` variants:
    - `KeyInputEvent(KeyInput)`,
    - `MouseInputEvent(MouseInput)`,
    - `Refresh`,
    - `Tick`
- Definitions for `KeyInput` and `MouseInput` can be found in `adapters/crossterm/input.rs`
- Instantiating an `AppEvent` starts a new OS thread and builds a `bounded mpsc channel` for 
communication between the newly created thread and the calling thread.
- The newly created thread:
    - uses the `crossterm` crate to capture key and mouse inputs to the terminal and sends them
    to the mpsc receiver
        - a timeout is supplied to the crossterm poll function to prevent it from blocking
    - sends tick events on the specified default `TICK_RATE`
    - sends refresh events on the specified default `REFRESH_RATE`

## Default control Flow
1. Build bounded `mpsc` channel with default `CHANNEL_SIZE`
2. Capture time instant `later` used to compute a time delta between the last refresh event and
the beginning of the loop
3. Spawn a new thread capturing all previously mentioned structures by value
4. Start loop in new thread
5. Capture time instant `now`
6. Compute time delta between `later` and `now` and compare to `REFRESH_RATE`
    - If greater than send RefreshEvent and go back to `5`
    - Else `7`
7. Poll `crossterm::event`
    - If `crossterm::event` is found then adapt to `KeyInput` | `MouseInput` and send over mpsc channel then go back to `5`.
    - If crossterm event queue is empty then continue attempting to poll for duration `TICK_RATE`
    - At the end of `TICK_RATE` send a `Tick` event over the mpsc channel

