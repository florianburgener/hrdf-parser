# strict graph
digraph {
    mindist=0.1

    node [shape=box]

    ATTRIBUT
    ATTRIBUT_DE
    ATTRIBUT_EN
    ATTRIBUT_FR
    ATTRIBUT_IT
    BAHNHOF
    BETRIEB_DE
    BETRIEB_EN
    BETRIEB_FR
    BETRIEB_IT
    BFKOORD_LV95
    BFKOORD_WGS
    BFPRIOS
    BHFART
    BHFART_60
    BITFELD
    DURCHBI
    ECKDATEN
    FEIERTAG
    FPLAN
    GLEIS
    GLEIS_LV95
    GLEIS_WGS
    GRENZHLT
    INFOTEXT_DE
    INFOTEXT_EN
    INFOTEXT_FR
    INFOTEXT_IT
    KMINFO
    LINIE
    METABHF
    RICHTUNG
    UMSTEIGB
    UMSTEIGL
    UMSTEIGV
    UMSTEIGZ
    ZEITVS
    ZUGART

    # ATTRIBUT, ATTRIBUT_DE, ATTRIBUT_EN, ATTRIBUT_FR, ATTRIBUT_IT
    ATTRIBUT_NODE [label="", shape=circle]
    ATTRIBUT_NODE -> ATTRIBUT [dir=back]
    ATTRIBUT_NODE -> ATTRIBUT_DE [dir=back]
    ATTRIBUT_NODE -> ATTRIBUT_EN [dir=back]
    ATTRIBUT_NODE -> ATTRIBUT_FR [dir=back]
    ATTRIBUT_NODE -> ATTRIBUT_IT [dir=back]

    # BETRIEB_DE, BETRIEB_EN, BETRIEB_FR, BETRIEB_IT
    BETRIEB_NODE [label="", shape=circle]
    BETRIEB_NODE -> BETRIEB_DE [dir=back]
    BETRIEB_NODE -> BETRIEB_EN [dir=back]
    BETRIEB_NODE -> BETRIEB_FR [dir=back]
    BETRIEB_NODE -> BETRIEB_IT [dir=back]

    # INFOTEXT_DE, INFOTEXT_EN, INFOTEXT_FR, INFOTEXT_IT
    INFOTEXT_NODE [label="", shape=circle]
    INFOTEXT_NODE -> INFOTEXT_DE [dir=back]
    INFOTEXT_NODE -> INFOTEXT_EN [dir=back]
    INFOTEXT_NODE -> INFOTEXT_FR [dir=back]
    INFOTEXT_NODE -> INFOTEXT_IT [dir=back]

    # Relationshipless
    RELATIONSHIPLESS_NODE [style=invis]
    RELATIONSHIPLESS_NODE -> ECKDATEN [style=invis]
    RELATIONSHIPLESS_NODE -> FEIERTAG [style=invis]
    RELATIONSHIPLESS_NODE -> GRENZHLT [style=invis]
    RELATIONSHIPLESS_NODE -> UMSTEIGL [style=invis]
    RELATIONSHIPLESS_NODE -> UMSTEIGV [style=invis]
    RELATIONSHIPLESS_NODE -> UMSTEIGZ [style=invis]
    RELATIONSHIPLESS_NODE -> ZEITVS [style=invis]

    # ------------------------------------------------------------------------------------------------
    # --- Relationships
    # ------------------------------------------------------------------------------------------------

    # BFKOORD_LV95
    BFKOORD_LV95 -> BAHNHOF

    # BFKOORD_WGS
    BFKOORD_WGS -> BAHNHOF

    # BFPRIOS
    BFPRIOS -> BAHNHOF

    # BHFART
    BHFART -> BAHNHOF

    # BHFART_60
    BHFART_60 -> BAHNHOF

    # DURCHBI
    DURCHBI -> BAHNHOF
    DURCHBI -> BITFELD
    DURCHBI -> FPLAN

    # FPLAN (TODO)
    FPLAN -> ATTRIBUT_NODE
    FPLAN -> BAHNHOF
    FPLAN -> BITFELD
    FPLAN -> LINIE
    FPLAN -> RICHTUNG
    FPLAN -> ZUGART

    # GLEIS
    GLEIS -> BITFELD
    GLEIS -> FPLAN

    # GLEIS_LV95
    GLEIS_LV95 -> BITFELD
    GLEIS_LV95 -> FPLAN

    # GLEIS_WGS
    GLEIS_WGS -> BITFELD
    GLEIS_WGS -> FPLAN

    # KMINFO
    KMINFO -> BAHNHOF

    # METABHF
    METABHF -> ATTRIBUT_NODE
    METABHF -> BAHNHOF

    # UMSTEIGB
    UMSTEIGB -> BAHNHOF
}
