Progression (23/38) :
* ATTRIBUT
* ATTRIBUT_DE
* ATTRIBUT_EN
* ATTRIBUT_FR
* ATTRIBUT_IT
* BAHNHOF
* BFKOORD_LV95
* BFKOORD_WGS
* BFPRIOS
* BITFELD
* DURCHBI
* ECKDATEN
* FEIERTAG
* GLEIS
* GLEIS_LV95
* GLEIS_WGS
* GRENZHLT (the file is empty)
* INFOTEXT_DE
* INFOTEXT_EN
* INFOTEXT_FR
* INFOTEXT_IT
* KMINFO
* RICHTUNG

TODO :

* thiserror
* Trouver un meilleur nom pour bit_field (actuellement : bit_field_id, etc.)
* Chunk loading des fichiers
* platforms_parser.rs => journey_platform_and_platforms_parser.rs ?
* Normaliser les fichiers parsing/*_parser.rs
* Changer TimetableKeyData vers key => value
* Nettoyer Hrdf.rs en utilisant des ResourceData pour regrouper data et indexes
  * Exemple : attribute_data: AttributeData, attribute_data.items(), attribute_data.primary_index()
  * Passer au singuler les parser

Next :
LINIE
ZUGART
