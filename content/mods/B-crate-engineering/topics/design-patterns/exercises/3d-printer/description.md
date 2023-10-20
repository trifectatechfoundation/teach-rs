An imaginary 3D printer uses filament to create all kinds of things.
Its states can be represented with the following state diagram:

```
                   ┌─────────────────┐
                   │                 │
                   │                 │   Reset
                   │      Idle       │◄────────────────────────────┐
         ┌────────►│                 │                             │
         │         │                 │                             │
         │         │                 │                             │
         │         └────────┬────────┘                             │
         │                  │                                      │
         │                  │                                      │
         │                  │ Start                                │
         │                  │                                      │
         │                  ▼                                      │
         │         ┌─────────────────┐                    ┌────────┴────────┐
         │         │                 │                    │                 │
         │         │                 │   Out of filament  │                 │
Product  │         │    Printing     ├──────────────────► │      Error      │
retrieved│         │                 │                    │                 │
         │         │                 │                    │                 │
         │         │                 │                    │                 │
         │         └────────┬────────┘                    └─────────────────┘
         │                  │
         │                  │ Product ready
         │                  │
         │                  ▼
         │         ┌─────────────────┐
         │         │                 │
         │         │                 │
         │         │  Product Ready  │
         └─────────┤                 │
                   │                 │
                   │                 │
                   └─────────────────┘
```

The printer boots in Idle state. Once a job is started, the printer enters the Printing state.
In printing state, it keeps on printing the product until either it is ready or the printer is out of filament.
If the printer is out of filament, the printer goes into Error state, which it can only come out of upon device reset.
If the product is ready, the printer goes to Product Ready state, and once the user retrieves the product, the printer goes back to Idle.

The printer can be represented in Rust using the typestate pattern as described during the lecture. This allows you to write a simple 3D printer driver. In `exercises/B/5-3d-printer`, a `Printer3D` struct is instantiated. Add methods corresponding to each of the traits, that simulate the state transitions by printing the state. A method simulating checking if the printer is out of filament is provided.

Of course, to make the printer more realistic, you can add more states and transitions.
