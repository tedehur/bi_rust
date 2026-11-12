# Feuille de route indicative

Les étapes ci-dessous dépendent des validations précédentes ; ce calendrier n’est ni un engagement ni une promesse de déploiement.

| Étape | Livrable | Critère de passage |
|---|---|---|
| 1. Cadrage utilisateur | Besoin, langues, usages et risques documentés | Revue par les personnes et organisations concernées |
| 2. Spécification | Format de messages et dictionnaire versionné | Cas de test et gestion des erreurs définis |
| 3. Prototype logiciel | Crate Rust, tests et documentation | Tests reproductibles ; limites connues |
| 4. Prototype matériel | Configuration vérifiée de la carte et de l’écran | Build et tests sur matériel identifié |
| 5. Mesures | Compression, énergie et communication | Méthodes et données brutes publiées |
| 6. Pilote supervisé | Essais non critiques | Consentement, procédures de repli, critères d’arrêt |
| 7. Revue indépendante | Rapport technique, sécurité, juridique et terrain | Décision documentée avant toute mise à l’échelle |

## Travaux techniques à venir

- Définir le dictionnaire et sa compatibilité entre versions.
- Définir l’encadrement des messages, l’intégrité et l’authentification.
- Choisir une carte et un environnement de build précis.
- Ajouter les adaptateurs matériels seulement après validation des broches et pilotes.
- Ajouter des benchmarks comparant au texte UTF-8 sur un corpus publié.
- Définir les procédures de clés, mises à jour, révocation, réparation et fin de vie.
