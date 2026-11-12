# Format de démonstration Rust — version 1

## Statut

Ce format rend testable une première idée de trame. Il ne prétend pas définir le format « Double Flag » historique ou commercial, et il n’effectue aucune compression, authentification ou transmission radio.

## Encodage

| Octet | Signification | Valeurs acceptées |
|---|---|---|
| 0 | Version | `0x01` |
| 1 | Classe | `0x00` quotidien ; `0x01` classe prioritaire illustrative |
| 2 | Identifiant du symbole | `0x00` à `0xFF` |

Exemple de classe quotidienne, symbole 7 : `01 00 07`. L’exemple fait 3 octets et ne doit pas être présenté comme une compression d’un message texte.

## Propriétés et limites

- La longueur et les valeurs de version/classe sont validées par `Frame::decode`.
- Un identifiant de symbole ne permet de retrouver un texte que si les nœuds partagent un dictionnaire versionné, absent de ce prototype.
- Aucune somme de contrôle, signature, chiffrement, horodatage, expiration, identifiant d’émetteur, fragmentation ou protection anti-rejeu.
- La catégorie « prioritaire » n’est pas une preuve d’origine et ne doit pas déclencher seule une action de secours.

## Mesure de compression à réaliser

Pour mesurer un taux, fixer un corpus représentatif et mesurer les octets totaux réellement stockés/transmis, y compris dictionnaire, version, métadonnées, erreurs, mises à jour et retransmissions. Comparer les mêmes messages en UTF-8 et dans le format proposé. Publier la formule, par exemple `1 - (taille encodée / taille de référence)`. Ne pas annoncer de pourcentage avant ces mesures.
