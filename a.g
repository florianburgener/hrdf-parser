digraph {
    layout="dot"
    overlap = false
    pad=0.5
    splines = ortho
    ranksep=1.5
    nodesep=1;
    rankdir=LR

    edge [penwidth=2]
    node [fontsize=16, shape="box", height=0.8, width=2]

    ATTRIBUT [color="red"]
    ATTRIBUT_DE [color="red"]
    ATTRIBUT_EN [color="red"]
    ATTRIBUT_FR [color="red"]
    ATTRIBUT_IT [color="red"]
    BAHNHOF [color="darkgreen"]
    BETRIEB_DE [color="darkorchid"]
    BETRIEB_EN [color="darkorchid"]
    BETRIEB_FR [color="darkorchid"]
    BETRIEB_IT [color="darkorchid"]
    BFKOORD_LV95
    BFKOORD_WGS
    BFPRIOS
    BHFART
    BHFART_60
    BITFELD [color="darkgoldenrod1"]
    DURCHBI
    ECKDATEN
    FEIERTAG
    FPLAN [color="blue"]
    GLEIS
    GLEIS_LV95
    GLEIS_WGS
    GRENZHLT
    INFOTEXT_DE [color="cyan3"]
    INFOTEXT_EN [color="cyan3"]
    INFOTEXT_FR [color="cyan3"]
    INFOTEXT_IT [color="cyan3"]
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
    ATTRIBUT_NODE [color="red", label=< <i>ATTRIBUT_NODE</i> >]
    ATTRIBUT_NODE -> ATTRIBUT [color="red"]
    ATTRIBUT_NODE -> ATTRIBUT_DE [color="red"]
    ATTRIBUT_NODE -> ATTRIBUT_EN [color="red"]
    ATTRIBUT_NODE -> ATTRIBUT_FR [color="red"]
    ATTRIBUT_NODE -> ATTRIBUT_IT [color="red"]

    # BETRIEB_DE, BETRIEB_EN, BETRIEB_FR, BETRIEB_IT
    BETRIEB_NODE [color="darkorchid", label=< <i>BETRIEB_NODE</i> >]
    BETRIEB_NODE -> BETRIEB_DE [color="darkorchid"]
    BETRIEB_NODE -> BETRIEB_EN [color="darkorchid"]
    BETRIEB_NODE -> BETRIEB_FR [color="darkorchid"]
    BETRIEB_NODE -> BETRIEB_IT [color="darkorchid"]

    # INFOTEXT_DE, INFOTEXT_EN, INFOTEXT_FR, INFOTEXT_IT
    INFOTEXT_NODE [color="cyan3", label=< <i>INFOTEXT_NODE</i> >]
    INFOTEXT_NODE -> INFOTEXT_DE [color="cyan3"]
    INFOTEXT_NODE -> INFOTEXT_EN [color="cyan3"]
    INFOTEXT_NODE -> INFOTEXT_FR [color="cyan3"]
    INFOTEXT_NODE -> INFOTEXT_IT [color="cyan3"]

    { rank=same; BAHNHOF, BITFELD, FPLAN }
    { rank=same; ATTRIBUT_NODE, INFOTEXT_NODE }

    ZEITVS -> BHFART_60 [style=invis]
    BETRIEB_NODE -> GRENZHLT [style=invis]
    { rank=same; FEIERTAG, GRENZHLT, UMSTEIGL, UMSTEIGV, UMSTEIGZ, ZEITVS }

    # ------------------------------------------------------------------------------------------------
    # --- Relationships
    # ------------------------------------------------------------------------------------------------

    # BFKOORD_LV95
    BFKOORD_LV95 -> BAHNHOF [color="darkgreen"]

    # BFKOORD_WGS
    BFKOORD_WGS -> BAHNHOF [color="darkgreen"]

    # BFPRIOS
    BFPRIOS -> BAHNHOF [color="darkgreen"]

    # BHFART
    BHFART -> BAHNHOF [color="darkgreen"]

    # BHFART_60
    BHFART_60 -> BAHNHOF [color="darkgreen"]

    # BITFELD
    BITFELD -> ECKDATEN # The BITFELD file uses the data indirectly.

    # DURCHBI
    DURCHBI -> BAHNHOF [color="darkgreen"]
    DURCHBI -> BITFELD [color="darkgoldenrod1"]
    DURCHBI -> FPLAN [color="blue"]

    # FPLAN
    FPLAN -> ATTRIBUT_NODE [color="red"]
    FPLAN -> BAHNHOF [color="darkgreen"]
    FPLAN -> BITFELD [color="darkgoldenrod1"]
    FPLAN -> INFOTEXT_NODE [color="cyan3"]
    FPLAN -> LINIE
    FPLAN -> RICHTUNG
    FPLAN -> ZUGART

    # GLEIS
    GLEIS -> BITFELD [color="darkgoldenrod1"]
    GLEIS -> FPLAN [color="blue"]

    # GLEIS_LV95
    GLEIS_LV95 -> BITFELD [color="darkgoldenrod1"]
    GLEIS_LV95 -> FPLAN [color="blue"]

    # GLEIS_WGS
    GLEIS_WGS -> BITFELD [color="darkgoldenrod1"]
    GLEIS_WGS -> FPLAN [color="blue"]

    # KMINFO
    KMINFO -> BAHNHOF [color="darkgreen"]

    # METABHF
    METABHF -> ATTRIBUT_NODE [color="red"]
    METABHF -> BAHNHOF [color="darkgreen"]

    # UMSTEIGB
    UMSTEIGB -> BAHNHOF [color="darkgreen"]
}
