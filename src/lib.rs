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

    /// Action exécutée lors de l'entrée dans un nouvel état.
    fn on_enter(&mut self, _state: Self::State) {}

    /// Action exécutée lors de la sortie d'un ancien état.
    fn on_exit(&mut self, _state: Self::State) {}
}

/// Gestionnaire générique de FSM (zéro allocation, zéro coût d'abstraction).
pub struct Fsm<M: StateMachine> {
    state: M::State,
    machine: M,
}

impl<M: StateMachine> Fsm<M> {
   
    /// Retourne l'état actuel de la machine.
    pub fn state(&self) -> M::State {
        self.state
    }


    /// Envoie un événement, calcule le nouvel état, et gère les hooks
    /// `on_exit` et `on_enter` si l'état change effectivement.
    pub fn dispatch(&mut self, event: M::Event) {
        let next_state = self.machine.step(self.state, event);
        if next_state != self.state {
            let old_state = self.state;
            self.machine.on_exit(old_state);   // Sortie de l'ancien état
            self.state = next_state;
            self.machine.on_enter(self.state); // Entrée dans le nouvel état
        }
    }

    /// Réinitialise la machine à son état initial en déclenchant
    /// les hooks de sortie et d'entrée si l'état actuel est différent.
    pub fn reset(&mut self) {
        let initial = M::initial_state();
        if initial != self.state {
            self.machine.on_exit(self.state);
            self.state = initial;
            self.machine.on_enter(self.state);
        }
    }

    /// Vérifie si la machine se trouve dans l'état spécifié.
    pub fn is_in_state(&self, state: M::State) -> bool {
        self.state == state
    }

    /// Crée une nouvelle instance de machine à états avec son état initial
    /// et déclenche le hook d'entrée correspondant.
    pub fn new(mut machine: M) -> Self {
        let state = M::initial_state();
        machine.on_enter(state); // Notifie l'entrée dans l'état initial
        Self { state, machine }
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

    struct TestMachineWithHooks {
        enter_count: u8,
        exit_count: u8,
    }

    impl StateMachine for TestMachineWithHooks {
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

        fn on_enter(&mut self, _state: Self::State) {
            self.enter_count += 1;
        }

        fn on_exit(&mut self, _state: Self::State) {
            self.exit_count += 1;
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

    #[test]
    fn test_is_in_state() {
        let mut fsm = Fsm::new(TestMachine);
        assert!(fsm.is_in_state(TestState::Off));
        assert!(!fsm.is_in_state(TestState::On));

        fsm.dispatch(TestEvent::Toggle);
        assert!(fsm.is_in_state(TestState::On));
    }

    #[test]
    fn test_hooks_execution() {
        let machine = TestMachineWithHooks {
            enter_count: 0,
            exit_count: 0,
        };
        let mut fsm = Fsm::new(machine);

        assert_eq!(fsm.machine.enter_count, 1);
        assert_eq!(fsm.machine.exit_count, 0);

        fsm.dispatch(TestEvent::Toggle);
        assert_eq!(fsm.state(), TestState::On);
        assert_eq!(fsm.machine.enter_count, 2);
        assert_eq!(fsm.machine.exit_count, 1);

        fsm.dispatch(TestEvent::Toggle);
        assert_eq!(fsm.state(), TestState::Off);
        assert_eq!(fsm.machine.enter_count, 3);
        assert_eq!(fsm.machine.exit_count, 2);
    }
}