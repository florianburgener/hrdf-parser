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

TODO :
* Ajouter thiserror

Algorithme de calcul du trajet le plus court (Done) :
* Maximum N connexions
* Seulement les trajets possibles
* On ne revient pas sur ses pas
* Ne pas considérer les arrêts où les changements sont désactivés
* Si une solution est trouvée, alors il faut arrêter les routes qui arriveraient de toute manière plus tard que celle-ci
* Empêcher de réemprunter le même type de trajet que précédemment (ex. sortir du 14 pour reprendre le 14 d'après)

Algorithme de calcul du trajet le plus court (TODO) :
* considérer le changement d'arrêt ("Genève, gare" vers "Genève")
* Considérer les temps de transferts lors d'une correspondance
* Mettre en place des optimisations
    * Faire attention aux conséquences qu'elles peuvent engendrer
* Renvoyer les résultats via une structure
* Pouvoir paginer les résultats
    * Récupérer N résults plus tôt
    * Récupérer N résults plus tard
* Gérer le problème du jour d'après (lundi 23 h 59 - mardi 00 h 00)
* Pouvoir préciser une heure de départ ou d'arrivée

Algorithme de calcul du trajet le plus court (Idées) :
* Pas de route sur 2 ou plusieurs jours
* Utiliser une HashMap pour chaque arrêt contenant le temps le plus court pour l'atteindre, si un trajet arrive à un arrêt après le meilleur de temps => terminated
    * Toujours pas de soluce à ça
* Avoir une HashMap (journey_id, stop_id) qui maintient en combien de correspondance l'arrêt a été atteint pour tuer ceux qui serait pas meilleur


Types de temps de transferts :
Quels sont les règles pour l'utilisation des temps de transferts ? Est-ce que c'est cumulatif ? Hiérarchique ? Avez-vous un pseudo-code à me fournir ou un document décrivant ceci ?

* UMSTEIGB
    * 2 temps : IC auf IC ou tous les autres cas
    * Faut-il considérer ces 2 temps séparement ? Dans les données les 2 valeur ont l'air d'être tout le temps les même.
* UMSTEIGL
    * Temps de transfert entre la ligne A et la ligne B
    * A => B uniquement ?
* UMSTEIGV
    * Temps de transfert entre 2 administrations
    * A <=> B ?
    * Dans quel cas cela arrive ?  J'aurais bien besoin d'en plus de précision sur la notion d'administration, je pense.
* UMSTEIGZ
    * Temps de transfert entre le trajet A et le trajet B
    * A => B uniquement ?
* FPLAN (\*CI/\*CO)
    * Temps de transfert lors de l'embarquement ou le débarquement d'un trajet
    * J'imagine des cas comme le passage par une zone de douane pour accéder à certaines voies
* METABHF
    * Temps de transfert pour se déplacer entre l'arrêt A et l'arrêt B
    * D'après ce que j'ai compris, ça permet principalement de connecter les réseaux de transports publics domestique aux réseaux ferroviaires
    * A => B ?

Existe t'il d'autre endroits définissant des temps de transferts ?
