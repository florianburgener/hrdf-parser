digraph {
    layout="dot"
    overlap=false
    pad=0.5
    splines=ortho
    ranksep=2
    nodesep=0.2
    rankdir=LR

    edge [penwidth=2]
    node [fontsize=22, shape="box", height=1, penwidth=1, width=3.7, margin=0]

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
            margin=40
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
            margin=40
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
            margin=40
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
            margin=40
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
            margin=40
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
    BFKOORD_ -> BAHNHOF

    # BFPRIOS
    BFPRIOS -> BAHNHOF

    # BHFART
    BHFART -> BAHNHOF

    # BITFELD
    BITFELD -> ECKDATEN  [style=dashed] # The BITFELD file uses the data indirectly.

    # DURCHBI
    DURCHBI -> BAHNHOF   [color="red"]
    DURCHBI -> BITFELD   [color="red"]
    DURCHBI -> FPLAN     [color="red"]

    # FPLAN
    FPLAN -> ATTRIBUT    [color="darkgreen"]
    FPLAN -> BAHNHOF     [color="darkgreen"]
    FPLAN -> BETRIEB_    [color="darkgreen"]
    FPLAN -> BITFELD     [color="darkgreen"]
    FPLAN -> INFOTEXT_   [color="darkgreen"]
    FPLAN -> LINIE       [color="darkgreen"]
    FPLAN -> RICHTUNG    [color="darkgreen"]
    FPLAN -> ZUGART      [color="darkgreen"]

    # GLEIS
    GLEIS -> BITFELD     [color="darkorchid"]
    GLEIS -> FPLAN       [color="darkorchid"]

    # KMINFO
    KMINFO -> BAHNHOF

    # METABHF
    METABHF -> ATTRIBUT  [color="cyan3"]
    METABHF -> BAHNHOF   [color="cyan3"]

    # UMSTEIGB
    UMSTEIGB -> BAHNHOF

    # UMSTEIGL
    UMSTEIGL -> BAHNHOF  [color="magenta"]
    UMSTEIGL -> BETRIEB_ [color="magenta"]
    UMSTEIGL -> LINIE [color="magenta"]

    # UMSTEIGV
    UMSTEIGV -> BAHNHOF  [color="navyblue"]
    UMSTEIGV -> BETRIEB_ [color="navyblue"]

    # UMSTEIGZ
    UMSTEIGZ -> BETRIEB_ [color="coral4"]
    UMSTEIGZ -> BITFELD  [color="coral4"]
    UMSTEIGZ -> FPLAN    [color="coral4"]
}
