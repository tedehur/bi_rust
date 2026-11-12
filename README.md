# bi_rust — BIBRAI / BIRUST

> Dossier de travail et prototype de recherche pour une communication locale sobre, orientée éducation et information d’urgence.

**État :** concept et démonstrateur logiciel préliminaire. Ce dépôt ne contient pas encore un firmware ESP32-S3 validé, un réseau mesh opérationnel, ni un système d’alerte certifié.

## Objectifs

- Explorer un format compact pour transmettre des identifiants de concepts plutôt que des phrases répétitives.
- Documenter une architecture de nœuds locaux à faible consommation, avec contenus éducatifs et messages prioritaires.
- Construire progressivement un prototype Rust vérifiable avant tout essai de terrain.

Les termes **Double Flag**, **V3/V4**, **BIBRAI** et **BIRUST** sont des noms de travail. Leur définition technique reste à valider avec les utilisateurs et les partenaires.

## Contenu

- [`docs/01-concept.md`](docs/01-concept.md) — besoin, objectifs et limites
- [`docs/02-architecture.md`](docs/02-architecture.md) — architecture conceptuelle et diagrammes Mermaid
- [`docs/03-prototype-format.md`](docs/03-prototype-format.md) — format de démonstration documenté
- [`docs/04-livre-blanc.md`](docs/04-livre-blanc.md) — livre blanc de cadrage
- [`docs/05-pilote-terrain.md`](docs/05-pilote-terrain.md) — protocole de pilote et checklist
- [`docs/06-notes-juridiques.md`](docs/06-notes-juridiques.md) — points à examiner par la conseillère juridique
- [`docs/roadmap.md`](docs/roadmap.md) — étapes de développement
- [`src/lib.rs`](src/lib.rs) — bibliothèque Rust de démonstration
- [`scripts/Build_Script.ps1`](scripts/Build_Script.ps1) — tests et compilation locale Rust

## Compiler et tester

Prérequis : Rust stable installé (`rustup` / `cargo`). Depuis la racine du dépôt :

```sh
cargo test
cargo build --release
```

Sous Windows PowerShell, le script du dépôt évite les hypothèses sur les chemins PlatformIO :

```powershell
Set-Location C:\chemin\vers\bi_rust
.\scripts\Build_Script.ps1
```

Le script ne flashe pas la carte. La cible matérielle exacte, le framework, la configuration PlatformIO/ESP-IDF, les broches de l’écran et la procédure d’upload doivent être confirmés avant d’ajouter une commande de flash. Ne lancez pas de flash avec une configuration supposée.

## Licence et propriété intellectuelle

Le code et la documentation de ce dépôt sont publiés sous la licence MIT, voir [`LICENSE`](LICENSE). Cette licence autorise la réutilisation, y compris commerciale, sous réserve de ses conditions. Il n’existe pas ici de clause « MIT mais propriétaire » ni de restriction spéciale sur l’algorithme.

Si une partie doit rester confidentielle ou être licenciée séparément, ne la publiez pas sous MIT avant d’avoir défini avec un conseil juridique les fichiers, titulaires de droits et conditions applicables. Aucune demande de brevet, marque déposée, certification, garantie de compression, garantie de sécurité ou performance terrain n’est revendiquée par ce dépôt.

## Sécurité et usage d’urgence

Ce démonstrateur n’est **pas** un service d’alerte certifié et ne doit pas remplacer les autorités, sirènes, réseaux officiels, procédures locales ou moyens de secours. Tout pilote doit être annoncé comme expérimental, testé hors situation réelle et supervisé par des responsables locaux. Les messages doivent être authentifiés, datés, dédupliqués et accompagnés d’une procédure de repli avant tout usage opérationnel.

## Contributions

Ouvrir une issue pour préciser le besoin, le matériel réellement utilisé, les concepts à encoder et les mesures attendues. Les contributions doivent distinguer résultats mesurés, hypothèses et propositions.
