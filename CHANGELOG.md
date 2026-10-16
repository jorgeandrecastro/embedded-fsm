# Changelog

Toutes les modifications notables de ce projet seront documentées dans ce fichier.

Le format est basé sur [Keep a Changelog](https://keepachangelog.com/fr/1.0.0/),
et ce projet adhère à la [Version Sémantique](https://semver.org/spec/v2.0.html).

## [0.8.0] - 2026-02-21

### Modifié
- **Simplification du modèle de transition :** Refactorisation de `Fsm::dispatch` pour garantir un comportement strict de type "un appel, au maximum une transition", éliminant les ambiguïtés liées au chaînage implicite.
- Suppression du paramètre générique constant `MAX_CHAINED` de la structure `Fsm` pour épurer l'API publique.
- Suppression de la variante d'erreur `TransitionError::InfiniteLoopDetected` devenue superflue.
- Utilisation de `saturating_add` pour le comptage des transitions afin d'éviter tout débordement du compteur (`u32`).
- Nettoyage et mise à jour de la suite de tests et de la documentation.

## [0.7.0] - 2026-02-21

### Ajouté
- **Protection contre les boucles infinies :** Introduction d'un paramètre générique constant à la compilation `const MAX_CHAINED: u32 = 10` sur la structure `Fsm` pour prémunir le système contre les récursions infinies et les anomalies de boucles en cascade.
- **Nouvelle variante d'erreur :** Ajout de `TransitionError::InfiniteLoopDetected` pour gérer explicitement les violations de sécurité lorsque les limites de transitions sont dépassées.

### Modifié
- Mise à jour de `Fsm::dispatch` et des signatures internes du runtime pour prendre en charge pleinement le paramètre générique constant `MAX_CHAINED` tout en préservant la rétrocompatibilité grâce au typage par défaut.
- Mise à jour de la documentation et des suites de tests pour couvrir les annotations de type strictes et la mécanique de détection de boucles infinies.

## [0.6.1] - 2026-02-10

### Corrigé
- Résolution des problèmes d'inférence de type du compilateur dans les suites de tests en appliquant des annotations de type explicites pour les instances de `Fsm`.
- Correction des erreurs de signature de `step` dans les définitions de traits et raffinement de la logique de `dispatch` pour garantir le déclenchement correct des hooks de sécurité.

## [0.6.0] - 2026-02-01

### Ajouté
- Version stable initiale de `embedded-fsm` prenant en charge `no_std`, zéro allocation, `forbid(unsafe_code)` et les hooks de cycle de vie (`on_enter`, `on_exit`, `on_transition`).