import pytest

from sentencex import segment

# ruff: noqa: E501
tests = [
    (
        "Après avoir été l'un des acteurs du projet génome humain, le Genoscope met aujourd'hui le cap vers la génomique environnementale. L'exploitation des données de séquences, prolongée par l'identification expérimentale des fonctions biologiques, notamment dans le domaine de la biocatalyse, ouvrent des perspectives de développements en biotechnologie industrielle.",
        [
            "Après avoir été l'un des acteurs du projet génome humain, le Genoscope met aujourd'hui le cap vers la génomique environnementale.",
            "L'exploitation des données de séquences, prolongée par l'identification expérimentale des fonctions biologiques, notamment dans le domaine de la biocatalyse, ouvrent des perspectives de développements en biotechnologie industrielle.",
        ],
    ),
    (
        "\"Airbus livrera comme prévu 30 appareils 380 cette année avec en ligne de mire l'objectif d'équilibre financier du programme en 2015\", a-t-il ajouté.",
        [
            "\"Airbus livrera comme prévu 30 appareils 380 cette année avec en ligne de mire l'objectif d'équilibre financier du programme en 2015\", a-t-il ajouté."
        ],
    ),
    (
        "À 11 heures ce matin, la direction ne décomptait que douze grévistes en tout sur la France : ce sont ceux du site de Saran (Loiret), dont l’effectif est de 809 salariés, dont la moitié d’intérimaires. Elle assure que ce mouvement « n’aura aucun impact sur les livraisons ».",
        [
            "À 11 heures ce matin, la direction ne décomptait que douze grévistes en tout sur la France : ce sont ceux du site de Saran (Loiret), dont l’effectif est de 809 salariés, dont la moitié d’intérimaires.",
            "Elle assure que ce mouvement « n’aura aucun impact sur les livraisons ».",
        ],
    ),
    (
        "Ce modèle permet d’afficher le texte « LL.AA.II.RR. » pour l’abréviation de « Leurs Altesses impériales et royales » avec son infobulle.",
        [
            "Ce modèle permet d’afficher le texte « LL.AA.II.RR. » pour l’abréviation de « Leurs Altesses impériales et royales » avec son infobulle."
        ],
    ),
    (
        "Les derniers ouvrages de Intercept Ltd. sont ici.",
        ["Les derniers ouvrages de Intercept Ltd. sont ici."],
    ),
]


@pytest.mark.parametrize("text,expected_sents", tests)
def test_segment(text, expected_sents):
    # assert len(segment("el", text)) <= len(expected_sents)
    assert list(segment("fr", text)) == expected_sents
