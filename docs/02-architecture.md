# Architecture conceptuelle

## Nœuds et flux

```mermaid
flowchart LR
    A[Équipe éditoriale locale] -->|contenus vérifiés| G[Passerelle ou nœud source]
    G <-->|liaison locale à définir| N1[Nœud communautaire 1]
    N1 <-->|liaison locale à définir| N2[Nœud communautaire 2]
    N2 <-->|liaison locale à définir| N3[Nœud école / bibliothèque]
    N1 --> U[Usagers et opérateurs]
    N2 --> U
    N3 --> U
    O[Canaux officiels d'information] -. hors projet, à consulter .-> U
```

Le dessin est fonctionnel et non une promesse de topologie mesh. Technologie radio, portée, fréquences autorisées, routage, alimentation et contraintes locales restent à spécifier et tester.

## Cycle de message proposé

```mermaid
sequenceDiagram
    participant E as Éditeur autorisé
    participant S as Nœud source
    participant R as Nœud relais (optionnel)
    participant D as Nœud d'affichage
    E->>S: Contenu + catégorie + horodatage
    S->>S: Validation et journalisation
    S->>R: Trame selon protocole à définir
    R->>D: Relais contrôlé (si configuré)
    D->>D: Vérification, déduplication, affichage
    D-->>E: Accusé de réception (à concevoir)
```

## Priorités

Les classes quotidiennes et prioritaires sont des catégories d’application. Une priorité logicielle ne garantit ni réception radio ni délai d’affichage. Avant tout usage réel, définir une politique d’autorisation, d’authentification, d’expiration, de révocation et de repli vers les canaux officiels.

## Couches à développer

| Couche | État dans ce dépôt | Travail nécessaire |
|---|---|---|
| Vocabulaire / identifiants | À définir avec les utilisateurs | Gouvernance et versionnement du dictionnaire |
| Trame Rust | Démonstrateur 3 octets | Format stable, compatibilité, erreurs et tests |
| Compression | Non implémentée | Corpus de référence, comparaison avec UTF-8 et métadonnées incluses |
| Transport radio | Non implémenté | Choix matériel, réglementation, tests de portée et d’interférence |
| Sécurité | Non implémentée | Modèle de menace, clés, signatures, anti-rejeu, mises à jour sûres |
| Affichage / ESP32-S3 | Non implémenté | Référence exacte de carte, écran, broches, mémoire et pilotes |
| Exploitation terrain | Non validée | Maintenance, énergie, formation, procédures et support |
