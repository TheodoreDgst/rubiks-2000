# Rubik's 2000
### Quand on en a marre de résoudre en 2000 coups, on Rubik's 2000 le coup!

## Fonctionnalités

- Détection de l'état via la webcam &#x231B;
- Résolution du cube en un nombre de coup minimum &#x231B;
- Affichage de la solution en 3D &#x231B;
- Tout cela dans une UI ergonomique! &#x231B;

## Avancement actuel:
Le projet n'est pas encore entièrement fonctionnel. Revenez dans 3 mois ;)
Le solver est en bonne voie!
La 3D est bien avancée.
La détéction du cube arrive!!!
L'UI arrivera plus tard...

## Utilisation

Pour utiliser **Rubik's 2000** :

Clonez le dépôt sur votre machine locale.
```
$ git clone git@github.com:TheodoreDgst/rubiks-2000.git
```

Pour tester l'affichage 3D:
```
$ cd rubiks-2000-first-graphic
$ cargo run
```
Pour accéder au code du solver:
```
$ cd rubiks-2000-v2
```
## Sources:

### 3D :
* La [documentation](https://docs.rs/kiss3d/latest/kiss3d/) de Kiss3d

### Camera :
* La [documentation](https://docs.rs/rscam/latest/rscam/) de RSCam

### Solver :
* Un bref [papier](https://www.jetir.org/papers/JETIR1906707.pdf) récapitulatif
* Un papier sur les [différents algorithmes](https://www.diva-portal.org/smash/get/diva2:816583/FULLTEXT01.pdf) existants, bref état de l'art
* Une [explication](http://cube20.org/) sur le "God's number"
* Le [papier original](https://www.jaapsch.net/puzzles/thistle.htm) de Thistlewaite ( utile pour les exemples de tables et l'explication des groupes)
* Des notions utiles sur la  [théorie des groupes](http://trucsmaths.free.fr/rubik_groupe.htm) avec le Rubik's Cube
* Le site [Jaap'z](https://www.jaapsch.net/puzzles/theory.htm#group) qui référence pas mal de liens utilies en lien avec le Rubik's Cube
* Une implémentations expliquée de Thistlewaite en [SQL](https://explainextended.com/2022/12/31/happy-new-year-14/#more-6978) qui nous a permis de mieux apréhender les algorithmes de réduction de groupe grâce aux schémas.
* Une implémentation en [rust](https://github.com/V-Wong/CubeSimRS) de cet algorithme

### UI :
* La [documentation](https://docs.rs/egui/latest/egui/) de egui, la lib que nous allons (probablement) utiliser pour l'UI

### Site-Web:
* Un [site](https://html5up.net/) utile pour l'html
* De la [doc](https://projet.eu.org/pedago/sin/ICN/2nde/1-html_css.pdf)
