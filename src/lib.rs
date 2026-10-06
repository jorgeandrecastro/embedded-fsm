// Copyright (C) 2026 Jorge Andre Castro
//
// Ce programme est un logiciel libre : vous pouvez le redistribuer et/ou le modifier
// selon les termes de la Licence Publique Générale GNU telle que publiée par la
// Free Software Foundation, soit la version 2 de la licence, soit (à votre convention)
// n'importe quelle version ultérieure.

//! # embedded-fsm
//!
//! Machine à états finis (FSM) ultra-légère, synchrone et sans allocation (`no_std`)
//! pour systèmes embarqués.
//!
//! Zéro dépendance, sans `unsafe`, conçue pour des transitions déterministes.
//!
//! ```rust
//! use embedded_fsm::{StateMachine, Fsm};
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! enum Etat { Veille, Actif }
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! enum Evenement { BoutonPressé, Timeout }
//!
//! struct MaMachine;
//!
//! impl StateMachine for MaMachine {
//!     type State = Etat;
//!     type Event = Evenement;
//!
//!     fn initial_state() -> Self::State {
//!         Etat::Veille
//!     }
//!
//!     fn step(&mut self, current: Self::State, event: Self::Event) -> Self::State {
//!         match (current, event) {
//!             (Etat::Veille, Evenement::BoutonPressé) => Etat::Actif,
//!             (Etat::Actif, Evenement::Timeout) => Etat::Veille,
//!             _ => current, // Reste dans l'état courant par défaut
//!         }
//!     }
//! }
//!
//! let mut fsm = Fsm::new(MaMachine);
//! assert_eq!(fsm.state(), Etat::Veille);
//! fsm.dispatch(Evenement::BoutonPressé);
//! assert_eq!(fsm.state(), Etat::Actif);
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Trait définissant le comportement d'une machine à états finis.
pub trait StateMachine {
    /// Le type représentant les différents états (généralement un `enum`).
    type State: Copy + PartialEq;
    /// Le type représentant les événements déclenchant les transitions.
    type Event;

    /// Retourne l'état initial de la machine.
    fn initial_state() -> Self::State;

    /// Calcule le nouvel état en fonction de l'état courant et de l'événement reçu.
    fn step(&mut self, current: Self::State, event: Self::Event) -> Self::State;
}

/// Gestionnaire générique de FSM (zéro allocation, zéro coût d'abstraction).
pub struct Fsm<M: StateMachine> {
    state: M::State,
    machine: M,
}

impl<M: StateMachine> Fsm<M> {
    /// Crée une nouvelle instance de machine à états avec son état initial.
    pub fn new(machine: M) -> Self {
        let state = M::initial_state();
        Self { state, machine }
    }

    /// Retourne l'état actuel de la machine.
    pub fn state(&self) -> M::State {
        self.state
    }

    /// Envoie un événement à la machine pour déclencher une transition potentielle.
    pub fn dispatch(&mut self, event: M::Event) {
        self.state = self.machine.step(self.state, event);
    }

    /// Réinitialise la machine à son état initial.
    pub fn reset(&mut self) {
        self.state = M::initial_state();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestState {
        Off,
        On,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestEvent {
        Toggle,
    }

    struct TestMachine;

    impl StateMachine for TestMachine {
        type State = TestState;
        type Event = TestEvent;

        fn initial_state() -> Self::State {
            TestState::Off
        }

        fn step(&mut self, current: Self::State, event: Self::Event) -> Self::State {
            match (current, event) {
                (TestState::Off, TestEvent::Toggle) => TestState::On,
                (TestState::On, TestEvent::Toggle) => TestState::Off,
            }
        }
    }

    #[test]
    fn test_fsm_transitions() {
        let mut fsm = Fsm::new(TestMachine);
        assert_eq!(fsm.state(), TestState::Off);

        fsm.dispatch(TestEvent::Toggle);
        assert_eq!(fsm.state(), TestState::On);

        fsm.dispatch(TestEvent::Toggle);
        assert_eq!(fsm.state(), TestState::Off);
    }
}