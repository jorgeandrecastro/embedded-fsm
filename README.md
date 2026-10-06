# embedded-fsm

[![Edition Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)](https://doc.rust-lang.org/edition-guide/rust-2021/)
![no_std](https://img.shields.io/badge/no__std-yes-success)
[![Licence: GPL-2.0-or-later](https://img.shields.io/badge/license-GPL--2.0--or--later-blue)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html)

Machine à états finis (FSM) synchrone, légère et sans allocation (`no_std`), conçue pour les systèmes embarqués.

## Fonctionnalités

- **Sans dépendance** : aucun allocateur requis.
- **Sûre** : le crate interdit l'utilisation de `unsafe`.
- **Déterministe** : les transitions sont définies par votre implémentation.

## Installation

Ajoutez `embedded-fsm` à la section `[dependencies]` de votre fichier `Cargo.toml` :

```toml
[dependencies]
embedded-fsm = "0.1"
```

## Exemple

```rust
use embedded_fsm::{Fsm, StateMachine};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Etat {
    Veille,
    Actif,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Evenement {
    BoutonPresse,
    Timeout,
}

struct MaMachine;

impl StateMachine for MaMachine {
    type State = Etat;
    type Event = Evenement;

    fn initial_state() -> Self::State {
        Etat::Veille
    }

    fn step(&mut self, current: Self::State, event: Self::Event) -> Self::State {
        match (current, event) {
            (Etat::Veille, Evenement::BoutonPresse) => Etat::Actif,
            (Etat::Actif, Evenement::Timeout) => Etat::Veille,
            _ => current,
        }
    }
}

fn main() {
    let mut fsm = Fsm::new(MaMachine);
    assert_eq!(fsm.state(), Etat::Veille);

    fsm.dispatch(Evenement::BoutonPresse);
    assert_eq!(fsm.state(), Etat::Actif);
}
```

`StateMachine` définit l'état initial et la logique de transition. `Fsm` conserve l'état courant ; `dispatch` lui transmet un événement et `reset` réinitialise la machine.

## Licence

GPL-2.0-or-later. Copyright (C) 2026 Jorge Andre Castro.
