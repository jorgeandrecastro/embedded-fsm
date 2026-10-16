// Copyright (C) 2026 Jorge Andre Castro
//
// Ce programme est un logiciel libre : vous pouvez le redistribuer et/ou le modifier
// selon les termes de la Licence Publique Générale GNU telle que publiée par la
// Free Software Foundation, soit la version 2 de la licence, soit (à votre convention)
// n'importe quelle version ultérieure.

//! # embedded-fsm
//!
//! Machine à états finis (FSM) ultra-légère, synchrone et sans allocation (`no_std`)
//! pour systèmes embarqués critiques (conformité ISO).
//!
//! Zéro dépendance, sans `unsafe`, conçue pour des transitions déterministes.
//!
//! ```rust
//! use embedded_fsm::{StateMachine, Fsm, TransitionError};
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
//!     type Action = ();
//!
//!     fn initial_state() -> Self::State {
//!         Etat::Veille
//!     }
//!
//!     fn step(&mut self, current: Self::State, event: Self::Event) -> Result<(Self::State, Option<Self::Action>), TransitionError> {
//!         match (current, event) {
//!             (Etat::Veille, Evenement::BoutonPressé) => Ok((Etat::Actif, None)),
//!             (Etat::Actif, Evenement::Timeout) => Ok((Etat::Veille, None)),
//!             _ => Err(TransitionError::InvalidTransition),
//!         }
//!     }
//! }
//!
//! let mut fsm: Fsm<MaMachine> = Fsm::new(MaMachine);
//! assert_eq!(fsm.state(), Etat::Veille);
//! assert!(fsm.is_in_state(Etat::Veille));
//!
//! assert!(fsm.dispatch(Evenement::BoutonPressé).is_ok());
//! assert_eq!(fsm.state(), Etat::Actif);
//! assert!(fsm.is_in_state(Etat::Actif));
//! assert_eq!(fsm.transition_count(), 1);
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Erreurs possibles lors d'une tentative de transition de la machine à états.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionError {
    /// La transition demandée n'est pas autorisée par la logique métier.
    InvalidTransition,
}

/// Trait définissant le comportement d'une machine à états finis.
pub trait StateMachine {
    /// Le type représentant les différents états (généralement un `enum`).
    type State: Copy + PartialEq;

    /// Le type représentant les événements déclenchant les transitions.
    type Event: Clone;

    /// Type d'action optionnelle retournée lors d'une transition.
    type Action;

    /// Retourne l'état initial de la machine.
    fn initial_state() -> Self::State;

    /// Calcule le nouvel état et l'action optionnelle associée, ou retourne une erreur.
    fn step(
        &mut self,
        _current: Self::State,
        event: Self::Event,
    ) -> Result<(Self::State, Option<Self::Action>), TransitionError>;

    /// Action exécutée lors de l'entrée dans un nouvel état.
    fn on_enter(&mut self, _state: Self::State) {}

    /// Action exécutée lors de la sortie d'un ancien état.
    fn on_exit(&mut self, _state: Self::State) {}

    /// Hook optionnel appelé à chaque transition réussie pour la traçabilité.
    fn on_transition(&mut self, _from: Self::State, _event: &Self::Event, _to: Self::State) {}
}

/// Gestionnaire générique de FSM (zéro allocation, zéro coût d'abstraction).
pub struct Fsm<M: StateMachine> {
    state: M::State,
    machine: M,
    transition_count: u32,
}

impl<M: StateMachine> Fsm<M> {
    /// Retourne l'état actuel de la machine.
    pub fn state(&self) -> M::State {
        self.state
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
        machine.on_enter(state);
        Self {
            state,
            machine,
            transition_count: 0,
        }
    }

    /// Retourne une référence mutable vers la machine interne.
    pub fn machine_mut(&mut self) -> &mut M {
        &mut self.machine
    }

    /// Retourne une référence immuable vers la machine interne.
    pub fn machine(&self) -> &M {
        &self.machine
    }

    /// Retourne le nombre total de transitions d'état réussies.
    pub fn transition_count(&self) -> u32 {
        self.transition_count
    }

    /// Traite un événement et effectue au maximum une transition.
    ///
    /// Chaque appel à `dispatch` exécute un seul `step`.
    /// Aucune transition en cascade n'est exécutée implicitement.
    #[must_use = "Le résultat de la transition doit être vérifié"]
    pub fn dispatch(
        &mut self,
        event: M::Event,
    ) -> Result<Option<M::Action>, TransitionError>
    where
        M::Event: Clone,
    {
        let event_for_hook = event.clone();

        // Calcule la transition sans modifier l'état courant.
        let (next_state, action) = self.machine.step(self.state, event)?;

        // Une transition n'est nécessaire que si l'état change.
        if next_state != self.state {
            let old_state = self.state;

            self.machine.on_exit(old_state);

            self.state = next_state;
            self.transition_count = self.transition_count.saturating_add(1);

            self.machine
                .on_transition(old_state, &event_for_hook, next_state);

            self.machine.on_enter(next_state);
        }

        Ok(action)
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
        InvalidEvent,
    }

    struct TestMachine;

    impl StateMachine for TestMachine {
        type State = TestState;
        type Event = TestEvent;
        type Action = ();

        fn initial_state() -> Self::State {
            TestState::Off
        }

        fn step(
            &mut self,
            _current: Self::State,
            event: Self::Event,
        ) -> Result<(Self::State, Option<Self::Action>), TransitionError> {
            match (_current, event) {
                (TestState::Off, TestEvent::Toggle) => Ok((TestState::On, None)),
                (TestState::On, TestEvent::Toggle) => Ok((TestState::Off, None)),
                _ => Err(TransitionError::InvalidTransition),
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
        type Action = ();

        fn initial_state() -> Self::State {
            TestState::Off
        }

        fn step(
            &mut self,
            _current: Self::State,
            event: Self::Event,
        ) -> Result<(Self::State, Option<Self::Action>), TransitionError> {
            match (_current, event) {
                (TestState::Off, TestEvent::Toggle) => Ok((TestState::On, None)),
                (TestState::On, TestEvent::Toggle) => Ok((TestState::Off, None)),
                _ => Err(TransitionError::InvalidTransition),
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
        assert_eq!(fsm.transition_count(), 0);

        assert!(fsm.dispatch(TestEvent::Toggle).is_ok());
        assert_eq!(fsm.state(), TestState::On);
        assert_eq!(fsm.transition_count(), 1);

        // Vérification du rejet explicite de l'événement invalide
        assert_eq!(
            fsm.dispatch(TestEvent::InvalidEvent),
            Err(TransitionError::InvalidTransition)
        );
        assert_eq!(fsm.state(), TestState::On);
    }

    #[test]
    fn test_is_in_state() {
        let mut fsm = Fsm::new(TestMachine);
        assert!(fsm.is_in_state(TestState::Off));
        assert!(!fsm.is_in_state(TestState::On));

        let _ = fsm.dispatch(TestEvent::Toggle);
        assert!(fsm.is_in_state(TestState::On));
    }

    #[test]
    fn test_hooks_execution() {
        let machine = TestMachineWithHooks {
            enter_count: 0,
            exit_count: 0,
        };
        let mut fsm = Fsm::new(machine);

        assert_eq!(fsm.machine().enter_count, 1);
        assert_eq!(fsm.machine().exit_count, 0);

        let _ = fsm.dispatch(TestEvent::Toggle);
        assert_eq!(fsm.state(), TestState::On);
        assert_eq!(fsm.machine().enter_count, 2);
        assert_eq!(fsm.machine().exit_count, 1);

        let _ = fsm.dispatch(TestEvent::Toggle);
        assert_eq!(fsm.state(), TestState::Off);
        assert_eq!(fsm.machine().enter_count, 3);
        assert_eq!(fsm.machine().exit_count, 2);
    }
}