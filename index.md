# Architecture Decision Record Starbot <!-- Replace with ADR title -->

This is a template for EdgeX Foundry ADR.
Ceci est l'Architecture Decision Record du projet Starbot sous le language Rust


### Submitters

Projet RUST pour EFREI
M2 Dev Manager Full Stack

- BOUKRI Samir
- VUAGNAT Vincent


## Change Log

List the changes to the document, incl. state, date, and PR URL.

State is one of: pending, approved, amended, deprecated.

Date is an ISO 8601 (YYYY-MM-DD) string.

PR is the pull request that submitted the change, including information such as the diff, contributors, and reviewers.

Format:

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

Describe:

- how the design is architecturally significant - warranting an ADR (versus simple issue and PR to fix a problem)

- the high level design approach (details described in the proposed design below)


## Proposed Design

Details of the design (without getting into implementation where possible).

Outline:

- services/modules to be impacted (changed)

- new services/modules to be added

- model and DTO impact (changes/additions/removals)

- API impact (changes/additions/removals)

- general configuration impact (establishment of new sections, changes/additions/removals)

- devops impact


## Considerations

Document alternatives, concerns, ancillary or related issues, questions that arose in debate of the ADR. 

Indicate if/how they were resolved or mollified.


## Decision

Document any agreed upon important implementation detail, caveats, future considerations, remaining or deferred design issues.

Document any part of the requirements not satisfied by the proposed design.


## Other Related ADRs

List any relevant ADRs - such as a design decision for a sub-component of a feature, a design deprecated as a result of this design, etc.. 

Format:

- \[ADR Title\]\(URL\) - Relevance


## References

List additional references.

Format:

- \[Title\]\(URL\)

