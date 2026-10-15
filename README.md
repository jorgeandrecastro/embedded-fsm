 <div align="center">

# `embedded-fsm`

**Lightweight, deterministic, allocation-free finite state machine for embedded systems.**

[![Rust 2021](https://img.shields.io/badge/Rust-2021-orange?logo=rust)](https://www.rust-lang.org/)
[![no_std](https://img.shields.io/badge/no__std-supported-success)](https://docs.rust-embedded.org/book/intro/no-std.html)
[![License: GPL-2.0-or-later](https://img.shields.io/badge/license-GPL--2.0--or--later-blue)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html)
[![Documentation](https://docs.rs/embedded-fsm/badge.svg)](https://docs.rs/embedded-fsm)

</div>

---

`embedded-fsm` is a deterministic, allocation-free finite state machine (FSM) designed for resource-constrained embedded systems. It supports `no_std` environments, forbids `unsafe` code, and provides explicit transition error handling.

## ✨ Why embedded-fsm?

- **Lightweight:** minimal implementation suitable for microcontrollers.
- **Deterministic:** explicit, reproducible state transitions.
- **Memory-efficient:** no heap allocation required.
- **Safety-oriented:** forbids `unsafe` code.
- **Lifecycle hooks:** supports `on_enter`, `on_exit`, and `on_transition`.
- **Explicit error handling:** invalid transitions return `TransitionError`.
- **Observable execution:** tracks successful transitions and exposes machine state.

## 🔧 Features

- Explicit current state.
- Event-driven transitions through `dispatch`.
- Typed transition results using `Result`.
- Machine reset through `reset`.
- Current-state inspection through `is_in_state`.
- Successful transition counting through `transition_count`.
- Lifecycle hooks for state entry, exit, and transitions.
- Access to the underlying state machine through `machine()` and `machine_mut()`.

## 📦 Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
embedded-fsm = "0.6"
```

## 🚀 Quick start

```rust
use embedded_fsm::{Fsm, StateMachine, TransitionError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Idle,
    Active,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Event {
    ButtonPressed,
    Timeout,
}

struct MyMachine {
    activation_count: u32,
}

impl StateMachine for MyMachine {
    type State = State;
    type Event = Event;
    type Action = ();

    fn initial_state() -> Self::State {
        State::Idle
    }

    fn step(
        &mut self,
        current: Self::State,
        event: Self::Event,
    ) -> Result<(Self::State, Option<Self::Action>), TransitionError> {
        match (current, event) {
            (State::Idle, Event::ButtonPressed) => {
                self.activation_count += 1;
                Ok((State::Active, None))
            }
            (State::Active, Event::Timeout) => {
                Ok((State::Idle, None))
            }
            _ => Err(TransitionError::InvalidTransition),
        }
    }
}

fn main() {
    let machine = MyMachine {
        activation_count: 0,
    };

    let mut fsm = Fsm::new(machine);

    assert_eq!(fsm.state(), State::Idle);
    assert_eq!(fsm.transition_count(), 0);

    assert!(fsm.dispatch(Event::ButtonPressed).is_ok());

    assert_eq!(fsm.state(), State::Active);
    assert!(fsm.is_in_state(State::Active));
    assert_eq!(fsm.transition_count(), 1);
    assert_eq!(fsm.machine().activation_count, 1);

    // Invalid transitions are rejected explicitly.
    assert_eq!(
        fsm.dispatch(Event::ButtonPressed),
        Err(TransitionError::InvalidTransition)
    );
}
```

## 🧩 Design

The crate separates the state machine's transition logic from the FSM runtime.

### `StateMachine`

Defines the machine's behavior:

- `initial_state()` — specifies the initial state.
- `step()` — evaluates an event and determines the next state.
- `on_enter`, `on_exit`, and `on_transition` — provide lifecycle hooks when implemented by the trait.

### `Fsm`

Manages execution and exposes the following operations:

| Method | Purpose |
|---|---|
| `dispatch(event)` | Processes an event and returns a transition result. |
| `state()` | Returns the current state. |
| `is_in_state(state)` | Checks whether the machine is in a given state. |
| `transition_count()` | Returns the number of successful transitions. |
| `machine()` | Provides shared access to the underlying machine. |
| `machine_mut()` | Provides mutable access to the underlying machine. |
| `reset()` | Resets the FSM to its initial state. |

## 🎯 Design goals

- Predictable state-transition behavior.
- No heap allocation.
- Compatibility with `no_std` environments.
- Explicit handling of invalid transitions.
- Minimal runtime overhead for embedded applications.

> **Scope:** These design properties support safety-oriented embedded development, but do not independently establish ISO compliance or certify the crate for safety-critical use.

## 📚 Documentation and source

- **API documentation:** https://docs.rs/embedded-fsm
- **GitHub repository:** https://github.com/jorgeandrecastro/embedded-fsm
- **Rust Embedded Book:** https://docs.rust-embedded.org/book/

## ⚖️ License

This project is distributed under the **GPL-2.0-or-later** license.

Copyright (C) 2026 Jorge Andre Castro.
