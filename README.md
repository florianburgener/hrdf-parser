Progression (34/38) :
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
* ZUGART
* ZEITVS (fichier pas utilisé dans le code)

Fichiers restants :
* FPLAN
* UMSTEIGL
* UMSTEIGV
* UMSTEIGZ

let primary_index = information_text_parser, stop_parser, transport_company_parser
let mut legacy_primary_index = attribute_parser, direction_parser, platform_parser, transport_type_parser

TODO :
* Terminer le parsing
* Remap les clés étrangères vers un i32
* thiserror
* Chunk loading des fichiers
