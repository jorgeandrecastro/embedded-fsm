# embedded-fsm

<div align="center">
  <img src="https://img.shields.io/badge/Rust-2021-orange?logo=rust" alt="Rust 2021" />
  <img src="https://img.shields.io/badge/no__std-yes-success" alt="no_std" />
  <img src="https://img.shields.io/badge/license-GPL--2.0--or--later-blue" alt="GPL-2.0-or-later" />
</div>

Machine à états finis (FSM) légère, synchrone et sans allocation pour les systèmes embarqués et les applications à ressources limitées.

> `embedded-fsm` est conçu pour être déterministe, sûr et compatible `no_std`, sans dépendance d'allocateur et sans `unsafe`.

## Pourquoi cette crate ?

- Minimaliste : très peu de code, très peu de surface d'API.
- Sans allocation : idéale pour les microcontrôleurs et les environnements stricts.
- Sûre par conception : interdiction explicite de `unsafe`.
- Déterministe : les transitions sont entièrement pilotées par votre logique métier.
- Flexible : hooks `on_enter` et `on_exit` pour gérer les effets de bord d'état.

## Fonctionnalités

- Gestion d'un état courant unique et explicite.
- Transition déclenchée par un événement (`dispatch`).
- Réinitialisation de la machine (`reset`).
- Vérification rapide de l'état (`is_in_state`).
- Support d'actions d'entrée et de sortie à chaque changement d'état.

## Installation

Ajoutez le crate au fichier `Cargo.toml` :

```toml
[dependencies]
embedded-fsm = "0.2"
```

## Exemple rapide

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

    assert!(fsm.is_in_state(Etat::Actif));
}
```

## Modèle de conception

`StateMachine` définit :

- l'état initial,
- la logique de transition,
- les hooks optionnels `on_enter` et `on_exit`.

`Fsm` conserve l'état courant et applique les transitions de façon déterministe.

- `dispatch` : traite un événement et passe dans le nouvel état si nécessaire.
- `is_in_state` : vérifie immédiatement l'état courant.
- `reset` : réinitialise la machine à son état initial.

## Licence

Ce projet est distribué sous la licence GPL-2.0-or-later.

Copyright (C) 2026 Jorge Andre Castro.
