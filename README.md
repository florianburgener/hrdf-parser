Progression (38/38) :
* ATTRIBUT
* ATTRIBUT_DE (fichier pas utilisé dans le code)
* ATTRIBUT_EN (fichier pas utilisé dans le code)
* ATTRIBUT_FR (fichier pas utilisé dans le code)
* ATTRIBUT_IT (fichier pas utilisé dans le code)
* BAHNHOF
* BETRIEB_DE
* BETRIEB_EN
* BETRIEB_FR
* BETRIEB_IT
* BFKOORD_LV95
* BFKOORD_WGS
* BFPRIOS
* BHFART (fichier pas utilisé dans le code)
* BHFART_60
* BITFELD
* DURCHBI
* ECKDATEN
* FEIERTAG
* FPLAN
* GLEIS
* GLEIS_LV95
* GLEIS_WGS
* GRENZHLT (fichier pas utilisé dans le code)
* INFOTEXT_DE
* INFOTEXT_EN
* INFOTEXT_FR
* INFOTEXT_IT
* KMINFO
* LINIE
* METABHF
* RICHTUNG
* UMSTEIGB
* UMSTEIGL
* UMSTEIGV
* UMSTEIGZ
* ZUGART
* ZEITVS (fichier pas utilisé dans le code)

Affichage : ATTRIBUT, BAHNHOF, BETRIEB_DE, BETRIEB_EN, BETRIEB_FR, BETRIEB_IT, BFKOORD_LV95, BFKOORD_WGS, DURCHBI, FEIERTAG, GLEIS, GLEIS_LV95, GLEIS_WGS
            INFOTEXT_DE, INFOTEXT_EN, INFOTEXT_FR, INFOTEXT_IT, LINIE, RICHTUNG, ZUGART
Affichage + Algorithme : ECKDATEN, FPLAN, METABHF
Algorithme : BITFELD, KMINFO, UMSTEIGB, UMSTEIGL, UMSTEIGV, UMSTEIGZ
Incertain : BFPRIOS, BHFART_60
Inutile : ATTRIBUT_DE, ATTRIBUT_EN, ATTRIBUT_FR, ATTRIBUT_IT, BHFART, GREENZHLT, ZEITVS

TODO :
* Lib :
  * Temps de correspondance
  * Ajouter thiserror
  * Automatiquement télécharger les dernières données
  * Pouvoir parse la dernière version HRDF
  * Logging
* Créer un projet API qui utilise la lib
* Créer l'application web

Algorithme de calcul du trajet le plus court (Résumé) :
* Maximum N connexions
* Seulement les trajets possibles
* Ne pas considérer les arrêts où les changements sont désactivés pour les connexions
* Blocage des boucles
  * On ne revient pas sur ses pas
  * Arrêter de suivre un trajet dès qu'il boucle sur lui-même
* Si une solution est trouvée, alors il faut arrêter les routes qui arriveraient de toute manière plus tard que celle-ci
* Empêcher de réemprunter le même type de trajet que précédemment (ex. sortir du 14 pour reprendre le 14 d'après)
* Filter les connexions et ne prendre que des trajets avec une route unique
  * Par exemple si la ligne 21 passe 10 fois, seulement les 2 trajets (1 dans chaque sens) arrivant le plus tôt sont considérés
* Considérer pour les connexions, tous les trajets passant jusqu'à une certaine heure (ex. heure de départ + 4 heures)
* Considérer le changement d'arrêt à pied (ex. "Genève, gare" vers "Genève")
* Pouvoir calculer un trajet sur 2 jours
* La route qui arrive en premier (le plus tôt) à un arrêt est la seule qui peut explorer les connexions
    * Les routes qui arrivent plus tard peuvent seulement suivre leur trajet jusqu'à la fin
* Si une journey a déjà été emprunté avec moins de connexions alors il n'est pas possible de l'emprunter lors de l'exploration des connexions
    * L'emprenter avec plus de connexions ne peut pas améliorer la solution

Algorithme de calcul du trajet le plus court (Problèmes) :
* Lent quand la solution requiert beaucoup de connexions
* Lent quand le temps d'arrivée de la solution est tard  (à partir de 3-4 heures plus tard que l'heure de départ)
* Ne maximise pas le temps de départ

Algorithme de calcul du trajet le plus court (Optionnel) :
* Changer comment les déplacements à pied sont stocker
  * Le premier et dernier déplacement ne doivent pas compter dans la comparaison de solution.
* Pouvoir paginer les résultats
    * Récupérer N résults plus tôt
    * Récupérer N résults plus tard
* Pouvoir préciser une heure de départ ou d'arrivée
* Renvoyer les résultats via une structure
* Ajouter un warning quand arrêt n'existe pas dans FPLAN mais dans BAHNHOF

Algorithme de calcul du trajet le plus court (Idées) :
* Forcer l'utilisation des trains quand l'arrêt d'arrivée est loin

* iter vs into_inter ?
* Supprimer les getters inutile
* Demander des précisions sur les temps d'échanges et version HRDF
* Alpha Shape => pas ouf

Catégorisation parser :

auto_increment :
map + collect :
vec_to_map before Storage::new :

