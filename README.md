HRDF 2.04 (38 fichiers) :
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

HRDF 2.0.5 :
GLEISE_LV95 (fichier pas utilisé dans le code)
GLEISE_WGS (fichier pas utilisé dans le code)

Affichage : ATTRIBUT, BAHNHOF, BETRIEB_DE, BETRIEB_EN, BETRIEB_FR, BETRIEB_IT, BFKOORD_LV95, BFKOORD_WGS, DURCHBI, FEIERTAG, GLEIS, GLEIS_LV95, GLEIS_WGS
            INFOTEXT_DE, INFOTEXT_EN, INFOTEXT_FR, INFOTEXT_IT, LINIE, RICHTUNG, ZUGART
Affichage + Algorithme : ECKDATEN, FPLAN, METABHF
Algorithme : BITFELD, KMINFO, UMSTEIGB, UMSTEIGL, UMSTEIGV, UMSTEIGZ
Incertain : BFPRIOS, BHFART_60
Inutile : ATTRIBUT_DE, ATTRIBUT_EN, ATTRIBUT_FR, ATTRIBUT_IT, BHFART, GREENZHLT, ZEITVS

TODO :
* Library : Data downloading from an URL + unzipping + better cache system
* Isochrone Map : Interpolation
* Isochrone Map : Form (select an origin point + time limit + isochrone interval)
* Library : Include exchange times
* Isochrone Map : Errors ?
* Library : Errors (no route found, no journey including the departure or arrival stop)
* Library : Logging ?
* Library : thiserror ?
* Isochrone Map : Make the display more pleasant
* Refactoring
  * iter vs into_inter ?
  * Delete unnecessary getters
* Comments

Catégorisation parser :
    auto_increment :
    map + collect :
    vec_to_map before Storage::new :
