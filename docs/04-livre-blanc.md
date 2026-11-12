# Livre blanc de cadrage — BIBRAI / BIRUST

**Version 0.1 — document de travail destiné à la discussion avec les partenaires et la conseillère juridique**  
**Date :** 12 novembre 2026  
**Statut :** concept en exploration ; pas de déploiement opérationnel validé.

## Résumé exécutif

BIBRAI/BIRUST explore une approche locale de diffusion de contenus éducatifs et communautaires, complétée par une classe de messages prioritaires. L’intuition centrale est de transmettre des identifiants compacts de concepts connus des appareils plutôt que de répéter des textes complets. Cette hypothèse pourrait réduire les octets transmis dans certains scénarios, mais le gain net dépend du dictionnaire, des métadonnées, des mises à jour et du corpus réel.

Le dépôt public contient une trame de démonstration en Rust et des documents de cadrage. Il ne contient pas encore une implémentation ESP32-S3, un protocole mesh, un mécanisme cryptographique ni des résultats terrain. Le projet doit donc suivre une progression par preuves : besoins, spécification, tests logiciels, tests matériels contrôlés, pilote supervisé, puis décision de mise à l’échelle.

## 1. Besoin et contexte

La connectivité et les ressources énergétiques peuvent être variables dans certaines zones rurales. Des outils locaux peuvent potentiellement rendre disponibles des contenus de référence sans dépendre d’une connexion permanente. Le besoin, les langues, les usages et les contraintes de chaque site doivent être établis avec les utilisateurs et responsables locaux plutôt que présumés.

## 2. Proposition de valeur à tester

- Accès local à des contenus éducatifs ou pratiques sélectionnés.
- Réutilisation d’un dictionnaire de concepts commun entre appareils.
- Possibilité de différencier des catégories de messages dans l’interface.
- Maintenance et documentation conçues pour des opérateurs locaux.

Ces éléments sont des hypothèses, pas des performances ou garanties déjà démontrées.

## 3. Principe de représentation compacte

Un dictionnaire partagé associerait un identifiant à un concept ou à une instruction prévalidée. Une trame transporterait l’identifiant, la catégorie et les informations de version requises. Pour être utile, l’économie doit être mesurée au niveau complet : transmission du dictionnaire, synchronisation, en-têtes, sécurité, répétitions et messages non présents dans le dictionnaire.

Le prototype `bi_rust` encode actuellement une version, une classe et un identifiant sur trois octets. Il ne compresse pas de texte et n’est pas le protocole final.

## 4. Architecture envisagée

Des nœuds alimentés localement pourraient stocker et afficher les contenus. Une passerelle ou un opérateur autorisé pourrait publier les mises à jour. Des liaisons radio locales peuvent être étudiées après choix de matériel et vérification réglementaire. Les schémas sont conceptuels : le routage mesh, la portée, la redondance et le fonctionnement hors réseau ne sont pas encore validés.

## 5. Sécurité, confiance et limites d’urgence

Un message prioritaire doit être authentifiable et clairement attribué avant d’être utilisé. Le futur système devra traiter les clés compromises, la révocation, l’horodatage, l’expiration, le rejeu, la perte de connectivité et les mises à jour malveillantes. Le chiffrement seul ne prouve ni l’autorité ni l’exactitude d’un message.

Jusqu’à validation indépendante et autorisation des autorités compétentes, le prototype ne doit jamais être utilisé comme canal officiel d’alerte ni remplacer les systèmes d’urgence existants. Un pilote peut afficher des scénarios simulés clairement marqués « TEST ».

## 6. Sobriété et modèle économique

L’économie d’énergie doit être mesurée sur le matériel exact, avec états de veille, réveils, affichage, transmission, température et vieillissement de batterie. Les coûts doivent inclure appareil, boîtier, énergie, installation, formation, données, réparation, remplacement et support. Aucun tarif, autonomie ou retour sur investissement n’est établi dans cette version.

## 7. Plan de validation

1. Entretiens de besoins et analyse des risques.
2. Spécification de la trame, du dictionnaire et de la gouvernance des mises à jour.
3. Tests unitaires et tests de compatibilité inter-version.
4. Mesures reproductibles de taille et d’énergie sur un prototype identifié.
5. Essais de communication contrôlés, sans rôle d’alerte publique.
6. Pilote communautaire avec consentement, supervision, procédures de repli et critères d’arrêt.
7. Évaluation indépendante avant toute décision de déploiement.

## 8. Propriété intellectuelle et ouverture

Le dépôt est publié sous MIT. Les partenaires peuvent réutiliser le code et la documentation conformément à cette licence. Il n’est pas possible de présenter cette publication comme une licence MIT assortie d’une réserve secrète sur les mêmes fichiers. Toute stratégie de brevet, secret d’affaires, marque, contrat de contribution ou licence duale doit être évaluée avant divulgation et documentée précisément par les titulaires de droits et leur conseil.

## 9. Décisions demandées aux partenaires

- Quel problème concret et quel groupe d’utilisateurs prioriser ?
- Qui est habilité à créer, vérifier et retirer les contenus ?
- Quel matériel exact sera testé et qui en assurera la maintenance ?
- Quels critères de succès, de coût et d’arrêt adopter ?
- Quel régime de données et de propriété intellectuelle est souhaité ?

## Conclusion

BIBRAI/BIRUST est une piste de recherche et de prototypage. Sa crédibilité dépendra de spécifications ouvertes, de mesures reproductibles, d’une gouvernance responsable et de tests en conditions contrôlées. La prochaine étape est de valider le besoin et de remplacer les hypothèses par des données vérifiées.
