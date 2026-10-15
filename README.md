<div align="center">
  <h1>embedded-fsm</h1>
</div>

<div align="center">
  <strong>FSM synchrone, ultra légère et sans allocation pour systèmes embarqués critiques.</strong>
</div>

<div align="center">
  <img src="https://img.shields.io/badge/Rust-2021-orange?logo=rust" alt="Rust 2021" />
  <img src="https://img.shields.io/badge/no__std-yes-success" alt="no_std" />
  <img src="https://img.shields.io/badge/license-GPL--2.0--or--later-blue" alt="GPL-2.0-or-later" />
  <img src="https://img.shields.io/badge/docs-rs-blue?logo=rust" alt="Docs.rs" />
</div>

> `embedded-fsm` est conçu pour être déterministe, sûr, compatible `no_std` et orienté conformité ISO, sans dépendance d'allocateur et sans `unsafe`.

## ✨ Pourquoi cette crate ?

- 🚀 Ultra légère : conception minimale, idéale pour les microcontrôleurs.
- 🧠 Déterministe : chaque transition suit une logique explicite et reproductible.
- 🛡️ Sûre par conception : interdiction explicite de `unsafe`.
- ♻️ Sans allocation : compatible avec les environnements les plus stricts.
- 🔌 Flexible et Rigoureuse : gestion explicite des erreurs de transition (`TransitionError`), hooks de cycle de vie (`on_enter`/`on_exit`/`on_transition`), suivi du nombre de transitions et accès direct à la machine interne.

## 🔧 Fonctionnalités

- État courant unique et explicite.
- Transition sécurisée déclenchée par un événement via `dispatch` (retournant un `Result` avec attribut `#[must_use]`).
- Réinitialisation de la machine avec `reset`.
- Vérification rapide de l'état avec `is_in_state`.
- Suivi du nombre total de transitions réussies avec `transition_count`.
- Hooks d'entrée, de sortie et de traçabilité pour chaque changement d'état.
- Accès sécurisé à la machine interne via `machine()` et `machine_mut()`.

## 📦 Installation

Ajoutez le crate au fichier `Cargo.toml` :

```toml
[dependencies]
embedded-fsm = "0.6"
```  


## 🚀 Exemple rapide

```Rust

use embedded_fsm::{Fsm, StateMachine, TransitionError};

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
    type Action = ();

    fn initial_state() -> Self::State {
        Etat::Veille
    }

    fn step(&mut self, current: Self::State, event: Self::Event) -> Result<(Self::State, Option<Self::Action>), TransitionError> {
        match (current, event) {
            (Etat::Veille, Evenement::BoutonPresse) => {
                self.compteur_activations += 1;
                Ok((Etat::Actif, None))
            }
            (Etat::Actif, Evenement::Timeout) => Ok((Etat::Veille, None)),
            _ => Err(TransitionError::InvalidTransition),
        }
    }
}

fn main() {
    let machine = MaMachine { compteur_activations: 0 };
    let mut fsm = Fsm::new(machine);

    assert_eq!(fsm.state(), Etat::Veille);
    assert_eq!(fsm.transition_count(), 0);

    assert!(fsm.dispatch(Evenement::BoutonPresse).is_ok());
    assert_eq!(fsm.state(), Etat::Actif);
    assert!(fsm.is_in_state(Etat::Actif));
    assert_eq!(fsm.transition_count(), 1);
    assert_eq!(fsm.machine().compteur_activations, 1);

    // Rejet explicite d'une transition invalide
    assert_eq!(fsm.dispatch(Evenement::BoutonPresse), Err(TransitionError::InvalidTransition));
}

``̀  
## 🧩 Modèle de conception

`StateMachine` définit :

* L'état initial (`initial_state`).
* La logique de transition avec gestion explicite des erreurs (`TransitionError`).
* Les hooks optionnels de cycle de vie (`on_enter`, `on_exit` et `on_transition`).

`Fsm` conserve l'état courant et applique les transitions de façon strictement déterministe :

* `dispatch` : traite un événement de manière sécurisée et retourne un `Result`.
* `is_in_state` : vérifie immédiatement l'état courant.
* `transition_count` : retourne le nombre total de transitions effectuées.
* `machine` / `machine_mut` : accèdent aux données internes de la machine.
* `reset` : réinitialise la machine à son état initial.

---

## 📚 Documentation

* **Documentation Rust** : [https://docs.rs/embedded-fsm](https://docs.rs/embedded-fsm)
* **Dépôt GitHub** : [https://github.com/jorgeandrecastro/embedded-fsm](https://github.com/jorgeandrecastro/embedded-fsm)

---

## ⚖️ Licence

Ce projet est distribué sous la licence GPL-2.0-or-later.

Copyright (C) 2026 Jorge Andre Castro.