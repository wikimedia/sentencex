import pytest

from sentencex import segment

# ruff: noqa: E501
tests = [
    (
        "A Lei do Sorteio (n.º 1860, de 4 de janeiro de 1908) introduziu o serviço militar obrigatório para as Forças Armadas do Brasil, implantado de fato em 1916, substituindo o recrutamento forçado, o antigo “tributo de sangue”, e permitindo a constituição de uma reserva.",
        [
            "A Lei do Sorteio (n.º 1860, de 4 de janeiro de 1908) introduziu o serviço militar obrigatório para as Forças Armadas do Brasil, implantado de fato em 1916, substituindo o recrutamento forçado, o antigo “tributo de sangue”, e permitindo a constituição de uma reserva."
        ],
    ),
     (
        "Os oficiais mantinham a disciplina pelo castigo corporal.[13] Na Marinha, isso resultou na Revolta da Chibata de 1910.[14]",
        [
            "Os oficiais mantinham a disciplina pelo castigo corporal.[13]",
            "Na Marinha, isso resultou na Revolta da Chibata de 1910.[14]"
        ],
    ),
     (
        "A nova legislação era a lei 2.556, de 26 de setembro de 1874, e o decreto 5.881, de 17 de fevereiro de 1875.[35]",
        [
            "A nova legislação era a lei 2.556, de 26 de setembro de 1874, e o decreto 5.881, de 17 de fevereiro de 1875.[35]",
        ],
    ),
]


@pytest.mark.parametrize("text,expected_sents", tests)
def test_segment(text, expected_sents):
    assert list(segment("pt", text)) == expected_sents
