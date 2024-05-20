# Starbot

## Description
Starbot est un projet d'entraînement au Rust dans lequel se trouve une simulation avec des robots qui parcourent une planète pour récupérer des ressources et les ramener à leur base.

## Table des Matières
- [Installation](#installation)
- [Usage](#usage)
- [Fonctionnalités](#fonctionnalités)


## Installation
### Prérequis
- Rust
- Cargo

## Usage
Exemple d'utilisation :

pour lancer le projet
- cargo run
  
pour terminer la simulation
- Ctrl + C 

pour lancer les tests
- cargo test

## Fonctionnalités

- La simulation se lance automatiquement au lancement du projet.
- Dans le terminal, il sera affiché :
  - La carte principale avec toutes les informations.
  - La carte personnelle de la base.
  - Les ressources obtenues par la base.
- Les robots se déplacent aléatoirement.
- Les robots ont une carte personnelle qu'ils mettent à jour en se déplaçant sur la carte ou en partageant leurs informations avec la base.
- Les robots peuvent ramasser une ressource et peuvent la déposer à la base.
- Les robots ne peuvent pas traverser un obstacle.
- La carte génère aléatoirement la position de la base et des ressources, mais la position des obstacles est générée avec une perlin noise.
- Les variables MAX_HEIGHT et MAX_WIDTH dans map.rs configurent la taille de la carte.
- Les robots sont gérés par une gestion en concurence avec programmation par acteur avec le framework actix 
