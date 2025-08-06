# 🧙 Little-RPG - Jeu de rôle CLI en Rust

![Rust Version](https://img.shields.io/badge/Rust-1.88-orange?logo=rust&logoColor=white&style=for-the-badge)

> Un projet en ligne de commande écrit en Rust, inspiré des mécaniques de RPG classiques et modernes (Dark Souls, roguelike, RPG papier). Le jeu combine narration, exploration, combat stratégique, loot et évolution du personnage.

---

## 🎯 Objectifs du projet

- Développer un RPG textuel complet en Rust
- Apprendre les bonnes pratiques en structuration de projet Rust
- Explorer des concepts avancés : inventaire, IA ennemie, donjons générés, sauvegarde JSON...
- Créer un gameplay profond, difficile à maîtriser mais accessible

---

## 🔧 Fonctionnalités prévues

### ✅ Phase 1 – Socle de gameplay
- [ ] Structuration des entités `Player` et `Enemy`
- [ ] Système de combat de base
- [ ] Expérience (XP) et montée de niveau
- [ ] Système de coups critiques
- [ ] Loot d’objets et d’XP à la mort d’un ennemi

### ⚔️ Phase 2 – Progression & profondeur
- [ ] Inventaire du joueur (objets, potions)
- [ ] Équipement (armes, armures, accessoires)
- [ ] Types d’ennemis :
  - Lambda
  - Nommés
  - Élites
  - Légendaires
- [ ] Statuts et altérations (empoisonné, brûlé, affaibli…)
- [ ] Ressources secondaires : vitalité, blocage, attaque
- [ ] Système de zones

### 🧭 Phase 3 – Exploration et navigation
- [ ] Menus de navigation (explorer, stats, quitter…)
- [ ] Donjons générés aléatoirement
- [ ] Salles de combat ou d’événement
- [ ] Points de repos (type feux de camp Dark Souls)

### 📜 Phase 4 – Immersion & narration
- [ ] Texte d’introduction et narration dynamique
- [ ] Succès / exploits (ex. battre un légendaire, finir sans soins…)
- [ ] Bestiaire consultable
- [ ] Cheat code `demonic_eye` (tue tous les ennemis dans la salle actuelle)

### 🏆 Phase 5 – Fin & post-game
- [ ] Sauvegarde et chargement de partie (via `serde_json`)
- [ ] Fin du jeu (victoire ou boss final)
- [ ] Déblocage du **mode Hardcore**
  - Suppression de la sauvegarde en cas de mort
  - Fin alternative

---

## 🛠️ Technologies utilisées

- Langage : **Rust**
- Librairies :
  - [`rand`](https://crates.io/crates/rand) – génération aléatoire
  - [`serde`](https://crates.io/crates/serde), [`serde_json`](https://crates.io/crates/serde_json) – sauvegarde JSON
  - (à venir) [`colored`] – mise en forme terminal

---

## 🚀 Lancer le jeu

```bash
cargo run
```

Le jeu est actuellement en cours de développement. Seules certaines fonctionnalités de base sont disponibles.

## 📌 À venir

- Interface de choix CLI plus fluide
- Ajout de mini-quêtes ou d'événements narratifs
- Gestion des builds, versions et changelogs

## 👨‍💻 Auteur

Projet réalisé par Q-Lukyss dans le cadre d’un apprentissage Rust appliqué à la conception de jeux vidéo en ligne de commande.