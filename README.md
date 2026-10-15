<div align="center">
  <h1>embedded-fsm</h1>
</div>

<div align="center">
  <strong>FSM synchrone, ultra légère et sans allocation pour systèmes embarqués.</strong>
</div>

<div align="center">
  <img src="https://img.shields.io/badge/Rust-2021-orange?logo=rust" alt="Rust 2021" />
  <img src="https://img.shields.io/badge/no__std-yes-success" alt="no_std" />
  <img src="https://img.shields.io/badge/license-GPL--2.0--or--later-blue" alt="GPL-2.0-or-later" />
  <img src="https://img.shields.io/badge/docs-rs-blue?logo=rust" alt="Docs.rs" />
</div>

> `embedded-fsm` est conçu pour être déterministe, sûr et compatible `no_std`, sans dépendance d'allocateur et sans `unsafe`.

## ✨ Pourquoi cette crate ?

- 🚀 Ultra légère : conception minimale, idéale pour les microcontrôleurs.
- 🧠 Déterministe : chaque transition suit une logique explicite et reproductible.
- 🛡️ Sûre par conception : interdiction explicite de `unsafe`.
- ♻️ Sans allocation : compatible avec les environnements les plus stricts.
- 🔌 Flexible : hooks `on_enter` et `on_exit`, accès direct à la machine interne, et logique de transition centralisée.

## 🔧 Fonctionnalités

- État courant unique et explicite.
- Transition déclenchée par un événement via `dispatch`.
- Réinitialisation de la machine avec `reset`.
- Vérification rapide de l'état avec `is_in_state`.
- Hooks d'entrée/sortie pour chaque changement d'état.
- Accès sécurisé à la machine interne via `machine()` et `machine_mut()`.

## 📦 Installation

Ajoutez le crate au fichier `Cargo.toml` :

```toml
[dependencies]
embedded-fsm = "0.3"
```

## 🚀 Exemple rapide

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

struct MaMachine {
    compteur_activations: u32,
}

impl StateMachine for MaMachine {
    type State = Etat;
    type Event = Evenement;

    fn initial_state() -> Self::State {
        Etat::Veille
    }

    fn step(&mut self, current: Self::State, event: Self::Event) -> Self::State {
        match (current, event) {
            (Etat::Veille, Evenement::BoutonPresse) => {
                self.compteur_activations += 1;
                Etat::Actif
            }
            (Etat::Actif, Evenement::Timeout) => Etat::Veille,
            _ => current,
        }
    }
}

fn main() {
    let machine = MaMachine { compteur_activations: 0 };
    let mut fsm = Fsm::new(machine);

    assert_eq!(fsm.state(), Etat::Veille);

    fsm.dispatch(Evenement::BoutonPresse);
    assert_eq!(fsm.state(), Etat::Actif);
    assert!(fsm.is_in_state(Etat::Actif));
    assert_eq!(fsm.machine().compteur_activations, 1);
}
```

## 🧩 Modèle de conception

`StateMachine` définit :

- l'état initial,
- la logique de transition,
- les hooks optionnels `on_enter` et `on_exit`.

`Fsm` conserve l'état courant et applique les transitions de façon déterministe :

- `dispatch` : traite un événement et passe dans un nouvel état si nécessaire,
- `is_in_state` : vérifie immédiatement l'état courant,
- `machine` / `machine_mut` : accèdent aux données internes de la machine,
- `reset` : réinitialise la machine à son état initial.

## 📚 Documentation

- Documentation Rust : https://docs.rs/embedded-fsm
- Dépôt GitHub : https://github.com/jorgeandrecastro/embedded-fsm

## ⚖️ Licence

Ce projet est distribué sous la licence GPL-2.0-or-later.

Copyright (C) 2026 Jorge Andre Castro.