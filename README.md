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
Algorithme : BFPRIOS, BITFELD, KMINFO, UMSTEIGB, UMSTEIGL, UMSTEIGV, UMSTEIGZ
Incertain : BHFART_60
Inutile : ATTRIBUT_DE, ATTRIBUT_EN, ATTRIBUT_FR, ATTRIBUT_IT, BHFART, GREENZHLT, ZEITVS

TODO :
* Ajouter thiserror

Algorithme de calcul du trajet le plus court (Résumé) :
*


* Maximum N connexions
* Seulement les trajets possibles
* On ne revient pas sur ses pas
* Ne pas considérer les arrêts où les changements sont désactivés
* Si une solution est trouvée, alors il faut arrêter les routes qui arriveraient de toute manière plus tard que celle-ci
* Empêcher de réemprunter le même type de trajet que précédemment (ex. sortir du 14 pour reprendre le 14 d'après)
* Mettre en place des optimisations
    * Faire attention aux conséquences qu'elles peuvent engendrer
* Considérer le changement d'arrêt ("Genève, gare" vers "Genève")
* Pouvoir calculer un trajet sur 2 jours

Algorithme de calcul du trajet le plus court (TODO) :
* Considérer les temps de transferts lors d'une correspondance

Algorithme de calcul du trajet le plus court (Optionnel) :
* Pouvoir paginer les résultats
    * Récupérer N résults plus tôt
    * Récupérer N résults plus tard
* Pouvoir préciser une heure de départ ou d'arrivée
* Renvoyer les résultats via une structure

3.21s => 1.65s

Problèmes de l'agorithme :
* Lent quand il y a beaucoup de connexions
* Lent quand le temps d'arrivée est relativement lointain
* Ne maximise pas le temps de départ



* Vérifier le nom des paramètre de closures + if let Some() = + match
* Ajouter les commentaires + noter les features
* Renommer routes vers connections ?
