digraph {
    layout="dot"
    #overlap=scalexy
    pad=0
    splines=ortho
    ranksep=5
    nodesep=0.5
    rankdir=LR

    edge [penwidth=3]
    node [fontsize=22, shape="box", height=1.5, penwidth=1, width=3.8, margin=0]

    ATTRIBUT [label=<ATTRIBUT*<BR />
        <FONT POINT-SIZE="18">*, *_DE, *_FR, *_IT, *_EN</FONT>
    >]
    BAHNHOF
    BETRIEB_ [label=<BETRIEB_*<BR />
        <FONT POINT-SIZE="18">*DE, *FR, *IT, *EN</FONT>
    >]
    BFKOORD_ [label=<BFKOORD_*<BR />
        <FONT POINT-SIZE="18">*WGS, *LV95</FONT>
    >]
    BFPRIOS
    BHFART [label=<BHFART<BR />
        <FONT POINT-SIZE="18">*, *_60</FONT>
    >]
    BITFELD
    DURCHBI
    ECKDATEN
    FEIERTAG
    FPLAN
    GLEIS [label=<GLEIS<BR />
        <FONT POINT-SIZE="18">*, *_LV95, *_WGS</FONT>
    >]
    GRENZHLT
    INFOTEXT_ [label=<INFOTEXT_*<BR />
        <FONT POINT-SIZE="18">*DE, *FR, *IT, *EN</FONT>
    >]
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

    # ------------------------------------------------------------------------------------------------
    # --- Subgraphs
    # ------------------------------------------------------------------------------------------------

    subgraph cluster_0 {
        color=black
        margin=2
        penwidth=2

        subgraph cluster_0 {
            color=gray94
            fontsize=34
            label=< <B>Transfer times</B> >
            margin=30
            style=filled


            UMSTEIGB
            UMSTEIGL
            UMSTEIGV
            UMSTEIGZ
        }
    }

    subgraph cluster_1 {
        color=black
        margin=2
        penwidth=2

        subgraph cluster_1 {
            color=gray94
            fontsize=34
            label=< <B>Time-relevant data</B> >
            margin=30
            style=filled

            BITFELD
            ECKDATEN
            FEIERTAG
            ZEITVS

            { rank=same; ECKDATEN, ZEITVS }
        }
    }

    subgraph cluster_2 {
        color=black
        margin=2
        penwidth=2

        subgraph cluster_2 {
            color=gray94
            fontsize=34
            label=< <B>Timetable data</B> >
            margin=30
            style=filled

            DURCHBI
            FPLAN
            GLEIS
        }
    }

    subgraph cluster_3 {
        color=black
        margin=2
        penwidth=2

        subgraph cluster_3 {
            color=gray94
            fontsize=34
            label=< <B>Master data</B> >
            margin=30
            style=filled

            ATTRIBUT
            BETRIEB_
            INFOTEXT_
            LINIE
            RICHTUNG
            ZUGART
        }
    }


    subgraph cluster_4 {
        color=black
        margin=2
        penwidth=2

        subgraph cluster_4 {
            color=gray94
            fontsize=34
            label=< <B>Stop data</B> >
            margin=30
            style=filled

            BAHNHOF
            BFKOORD_
            BFPRIOS
            BHFART
            KMINFO
            METABHF
        }
    }

    # ------------------------------------------------------------------------------------------------
    # --- Relationships
    # ------------------------------------------------------------------------------------------------

    # BFKOORD_
    BFKOORD_ -> BAHNHOF  [color="springgreen2"]

    # BFPRIOS
    BFPRIOS -> BAHNHOF   [color="olivedrab"]

    # BHFART
    BHFART -> BAHNHOF    [color="forestgreen"]

    # BITFELD
    BITFELD -> ECKDATEN  [style=dashed] # The BITFELD file uses the data indirectly.

    # DURCHBI
    DURCHBI -> BAHNHOF   [color="turquoise4"]
    DURCHBI -> BITFELD   [color="turquoise4"]
    DURCHBI -> FPLAN     [color="turquoise4"]

    # FPLAN
    FPLAN -> ATTRIBUT    [color="mediumblue"]
    FPLAN -> BAHNHOF     [color="mediumblue"]
    FPLAN -> BETRIEB_    [color="mediumblue"]
    FPLAN -> BITFELD     [color="mediumblue"]
    FPLAN -> INFOTEXT_   [color="mediumblue"]
    FPLAN -> LINIE       [color="mediumblue"]
    FPLAN -> RICHTUNG    [color="mediumblue"]
    FPLAN -> ZUGART      [color="mediumblue"]

    # GLEIS
    GLEIS -> BITFELD     [color="slateblue1"]
    GLEIS -> FPLAN       [color="slateblue1"]

    # KMINFO
    KMINFO -> BAHNHOF    [color="limegreen"]

    # METABHF
    METABHF -> ATTRIBUT  [color="darkgreen"]
    METABHF -> BAHNHOF   [color="darkgreen"]

    # UMSTEIGB
    UMSTEIGB -> BAHNHOF  [color="maroon"]

    # UMSTEIGL
    UMSTEIGL -> BAHNHOF  [color="orange"]
    UMSTEIGL -> BETRIEB_ [color="orange"]
    UMSTEIGL -> LINIE    [color="orange"]

    # UMSTEIGV
    UMSTEIGV -> BAHNHOF  [color="red"]
    UMSTEIGV -> BETRIEB_ [color="red", constraint=false]

    # UMSTEIGZ
    UMSTEIGZ -> BETRIEB_ [color="violetred"]
    UMSTEIGZ -> BITFELD  [color="violetred"]
    UMSTEIGZ -> FPLAN    [color="violetred"]
}
