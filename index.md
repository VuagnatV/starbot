# Architecture Decision Record Starbot

### Submitters

Projet RUST pour EFREI
M2 Dev Manager Full Stack

- BOUKRI Samir
- VUAGNAT Vincent


## Change Log

- \[Ajout des tests et modification de l'index \] 2024-05-20
- \[Ajout de l'index et fix merge de la map \] 2024-05-20
- \[Modification pour split le main \] 2024-05-20
- \[Ajout du stockage de la map ainsi que fix de la map \] 2024-05-20
- \[Ajout du système de retrait des ressources \] 2024-05-20
- \[Ajout des multiples ressources ainsi que leurs générations \] 2024-05-20
- \[Ajout d'un système de mapping personnel \] 2024-05-20
- \[Creation d'une V2 pour réorganisation \] 2024-05-20
- \[Modification des imports pour tests unitaire et création de lib.rs\] 2024-05-16
- \[Modification des id des robots\] 2024-05-16
- \[Suppression de logs directory\] 2024-05-16
- \[Ajout de la génération de la base, des minerai ainsi que de l'énergie\] 2024-05-16
- \[Ajout d'affichage de la map et fication de speed random pour noise\] 2024-05-16
- \[Ajout des obstacle lors de la génération de la map\] 2024-05-15
- \[Ajout d'une instanciation de la map\] 2024-05-15
- \[Génération du projet\] 2024-05-13



## Referenced Use Case(s)

Pour pouvoir lancer le projet il suffit de lander un terminal et de lancer la commande d'ecéxution.

Commande:

- \[cargo run\]


## Context

L'objectif du projet est de créer un essaim de robot autonome spécialisés pour l'exploration spatiale et la recherche astrobiologique. Ces robots collaborent pour mener des missions d'exploration et de recherche sur des corps céleste(planètes, lunes, astéroïdes) afin de recueillir des données sur la géologie, la chimie ainsi que les potentiels signes de vie.


## Proposed Design

Outline:

- Des robots autonomes capable de se déplacer, de récolter des énergies ainsi que des ressources et de pouvoir cartographier la carte. Il peuvent ramener ces informations à la Base.

- La Base permet de recueillir les informations de la map par les robot ainsi que de recevoir les énergies ainsi que les minerais.

- La gestion d'une map aléatoire avec des obstacles, des minerais ainsi que des énergies.


## Decision

Lors de la création de ce projet, le système d'implémentation de la map ainsi que des robots n'étaient pas une grande difficultés, la plus grande difficulté rencontré à été la gestion des threads safe afin de convenir d'un code fonctionnel et d'un meilleur design pattern.
Concernant le path finding, cette fonctionnalité n'a pas pu être implémenter à cause d'un manque de connaissance pour faire fonctionner cette fonctionnalité. 