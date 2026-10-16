
# embedded-fsm

**Machine à états finis légère, déterministe et sans allocation pour les systèmes embarqués.**

`embedded-fsm` est une machine à états finis (FSM) déterministe et sans allocation, conçue pour les systèmes embarqués à ressources limitées. Elle prend en charge les environnements `no_std`, interdit le code `unsafe` et fournit une gestion explicite des erreurs de transition.

## ✨ Pourquoi embedded-fsm ?

- **Légère :** implémentation minimale adaptée aux microcontrôleurs.
- **Déterministe :** transitions d'état explicites et reproductibles.
- **Économe en mémoire :** aucune allocation sur le tas (*heap*).
- **Orientée sûreté :** interdit le code `unsafe`.
- **Hooks de cycle de vie :** prend en charge `on_enter`, `on_exit` et `on_transition`.
- **Gestion explicite des erreurs :** les transitions invalides renvoient une erreur `TransitionError`.
- **Exécution observable :** comptabilise les transitions réussies et expose l'état courant de la machine.
- **Protection contre les boucles infinies :** paramètre générique constant configurable (`MAX_CHAINED`) pour limiter les cascades de transitions et prévenir les boucles infinies.

## 🔧 Fonctionnalités

- Gestion explicite de l'état courant.
- Transitions pilotées par des événements via `dispatch`.
- Résultats de transition typés à l'aide de `Result`.
- Réinitialisation de la machine via `reset`.
- Vérification de l'état courant via `is_in_state`.
- Comptage des transitions réussies via `transition_count`.
- Hooks de cycle de vie lors de l'entrée dans un état, de la sortie d'un état et des transitions.
- Accès à la machine sous-jacente via `machine()` et `machine_mut()`.
- Limite de sécurité définie à la compilation via `const MAX_CHAINED`.

## 📦 Installation

Ajoutez la dépendance à votre fichier `Cargo.toml` :

```toml
[dependencies]
embedded-fsm = "0.7"
```

## 🚀 Démarrage rapide

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

    let mut fsm: Fsm<MyMachine> = Fsm::new(machine);

    assert_eq!(fsm.state(), State::Idle);
    assert_eq!(fsm.transition_count(), 0);

    assert!(fsm.dispatch(Event::ButtonPressed).is_ok());

    assert_eq!(fsm.state(), State::Active);
    assert!(fsm.is_in_state(State::Active));
    assert_eq!(fsm.transition_count(), 1);
    assert_eq!(fsm.machine().activation_count, 1);

    // Les transitions invalides sont explicitement rejetées.
    assert_eq!(
        fsm.dispatch(Event::ButtonPressed),
        Err(TransitionError::InvalidTransition)
    );
}
```

## 🧩 Architecture

La bibliothèque sépare la logique de transition de la machine à états de son moteur d'exécution.

### `StateMachine`

Définit le comportement de la machine :

- `initial_state()` — définit l'état initial.
- `step()` — évalue un événement et détermine l'état suivant.
- `on_enter`, `on_exit` et `on_transition` — fournissent des hooks de cycle de vie lorsqu'ils sont implémentés par le trait.

### `Fsm`

Gère l'exécution et expose les opérations suivantes :

| Méthode | Description |
|---|---|
| `dispatch(event)` | Traite un événement et renvoie le résultat de la transition. |
| `state()` | Renvoie l'état courant. |
| `is_in_state(state)` | Vérifie si la machine se trouve dans un état donné. |
| `transition_count()` | Renvoie le nombre de transitions réussies. |
| `machine()` | Fournit un accès en lecture à la machine sous-jacente. |
| `machine_mut()` | Fournit un accès mutable à la machine sous-jacente. |
| `reset()` | Réinitialise la FSM à son état initial. |

## 🎯 Objectifs de conception

- Garantir un comportement prévisible des transitions d'état.
- Éviter toute allocation sur le tas.
- Assurer la compatibilité avec les environnements `no_std`.
- Gérer explicitement les transitions invalides.
- Minimiser la surcharge à l'exécution pour les applications embarquées.

> **Périmètre :** ces propriétés de conception favorisent le développement de systèmes embarqués soumis à des exigences de sûreté. Elles ne démontrent toutefois pas, à elles seules, la conformité à une norme ISO et ne constituent pas une certification pour les systèmes critiques.

## 📚 Documentation et code source

- **Documentation de l'API :** [docs.rs/embedded-fsm](https://docs.rs/embedded-fsm)
- **Dépôt GitHub :** [github.com/jorgeandrecastro/embedded-fsm](https://github.com/jorgeandrecastro/embedded-fsm)
- **Livre Rust Embedded :** [docs.rust-embedded.org/book](https://docs.rust-embedded.org/book/)

## 📝 Changelog

Pour consulter l'historique détaillé de toutes les modifications, reportez-vous au fichier [CHANGELOG.md](CHANGELOG.md).

## ⚖️ Licence

Ce projet est distribué sous licence **GPL-2.0-or-later**.

Copyright (C) 2026 Jorge Andre Castro.
