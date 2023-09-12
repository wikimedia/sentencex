import pytest

from sentencex import segment

# ruff: noqa: E501

tests = [
    (
        "„Ich habe heute keine Zeit“, sagte die Frau und flüsterte leise: „Und auch keine Lust.“ Wir haben 1.000.000 Euro.",
        [
            "„Ich habe heute keine Zeit“, sagte die Frau und flüsterte leise: „Und auch keine Lust.“",
            "Wir haben 1.000.000 Euro.",
        ],
    ),
    (
        "Es gibt jedoch einige Vorsichtsmaßnahmen, die Du ergreifen kannst, z. B. ist es sehr empfehlenswert, dass Du Dein Zuhause von allem Junkfood befreist.",
        [
            "Es gibt jedoch einige Vorsichtsmaßnahmen, die Du ergreifen kannst, z. B. ist es sehr empfehlenswert, dass Du Dein Zuhause von allem Junkfood befreist."
        ],
    ),
    (
        "Was sind die Konsequenzen der Abstimmung vom 12. Juni?",
        ["Was sind die Konsequenzen der Abstimmung vom 12. Juni?"],
    ),
    (
        "„Ich habe heute keine Zeit“, sagte die Frau und flüsterte leise: „Und auch keine Lust.“ Wir haben 1.000.000 Euro.",
        [
            "„Ich habe heute keine Zeit“, sagte die Frau und flüsterte leise: „Und auch keine Lust.“",
            "Wir haben 1.000.000 Euro.",
        ],
    ),
    pytest.param(
        "Thomas sagte: ,,Wann kommst zu mir?” ,,Das weiß ich noch nicht“, antwortete Susi, ,,wahrscheinlich am Sonntag.“ Wir haben 1.000.000 Euro.",
        [
            "Thomas sagte: ,,Wann kommst zu mir?” ,,Das weiß ich noch nicht“, antwortete Susi, ,,wahrscheinlich am Sonntag.“",
            "Wir haben 1.000.000 Euro.",
        ],
        marks=pytest.mark.xfail,
    ),
    (
        "„Lass uns jetzt essen gehen!“, sagte die Mutter zu ihrer Freundin, „am besten zum Italiener.“",
        [
            "„Lass uns jetzt essen gehen!“, sagte die Mutter zu ihrer Freundin, „am besten zum Italiener.“"
        ],
    ),
    ("Wir haben 1.000.000 Euro.", ["Wir haben 1.000.000 Euro."]),
    ("Sie bekommen 3,50 Euro zurück.", ["Sie bekommen 3,50 Euro zurück."]),
    ("Dafür brauchen wir 5,5 Stunden.", ["Dafür brauchen wir 5,5 Stunden."]),
    ("Bitte überweisen Sie 5.300,25 Euro.", ["Bitte überweisen Sie 5.300,25 Euro."]),
    pytest.param(
        "1. Dies ist eine Punkteliste.",
        ["1. Dies ist eine Punkteliste."],
        marks=pytest.mark.xfail,
    ),
    ("Wir trafen Dr. med. Meyer in der Stadt.", ["Wir trafen Dr. med. Meyer in der Stadt."]),
    (
        "Wir brauchen Getränke, z. B. Wasser, Saft, Bier usw.",
        ["Wir brauchen Getränke, z. B. Wasser, Saft, Bier usw."],
    ),
    ("Ich kann u.a. Spanisch sprechen.", ["Ich kann u.a. Spanisch sprechen."]),
    ("Frau Prof. Schulze ist z. Z. nicht da.", ["Frau Prof. Schulze ist z. Z. nicht da."]),
    (
        "Sie erhalten ein neues Bank-Statement bzw. ein neues Schreiben.",
        ["Sie erhalten ein neues Bank-Statement bzw. ein neues Schreiben."],
    ),
    ("Z. T. ist die Lieferung unvollständig.", ["Z. T. ist die Lieferung unvollständig."]),
    ("Das finden Sie auf S. 225.", ["Das finden Sie auf S. 225."]),
    ("Sie besucht eine kath. Schule.", ["Sie besucht eine kath. Schule."]),
    (
        "Wir benötigen Zeitungen, Zeitschriften u. Ä. für unser Projekt.",
        ["Wir benötigen Zeitungen, Zeitschriften u. Ä. für unser Projekt."],
    ),
    (
        "Das steht auf S. 23, s. vorherige Anmerkung.",
        ["Das steht auf S. 23, s. vorherige Anmerkung."],
    ),
    (
        "Dies ist meine Adresse: Dr. Meier, Berliner Str. 5, 21234 Bremen.",
        ["Dies ist meine Adresse: Dr. Meier, Berliner Str. 5, 21234 Bremen."],
    ),
    (
        "Er sagte: „Hallo, wie geht´s Ihnen, Frau Prof. Müller?“",
        ["Er sagte: „Hallo, wie geht´s Ihnen, Frau Prof. Müller?“"],
    ),
    pytest.param(
        "Fit in vier Wochen\n\nDeine Anleitung für eine reine Ernährung und ein gesünderes und glücklicheres Leben\n\nRECHTLICHE HINWEISE\n\nOhne die ausdrückliche schriftliche Genehmigung der Eigentümerin von instafemmefitness, Anna Anderson, darf dieses E-Book weder teilweise noch in vollem Umfang reproduziert, gespeichert, kopiert oder auf irgendeine Weise übertragen werden. Wenn Du das E-Book auf einem öffentlich zugänglichen Computer ausdruckst, musst Du es nach dem Ausdrucken von dem Computer löschen. Jedes E-Book wird mit einem Benutzernamen und Transaktionsinformationen versehen.\n\nVerstöße gegen dieses Urheberrecht werden im vollen gesetzlichen Umfang geltend gemacht. Obgleich die Autorin und Herausgeberin alle Anstrengungen unternommen hat, sicherzustellen, dass die Informationen in diesem Buch zum Zeitpunkt der Drucklegung korrekt sind, übernimmt die Autorin und Herausgeberin keine Haftung für etwaige Verluste, Schäden oder Störungen, die durch Fehler oder Auslassungen in Folge von Fahrlässigkeit, zufälligen Umständen oder sonstigen Ursachen entstehen, und lehnt hiermit jedwede solche Haftung ab.\n\nDieses Buch ist kein Ersatz für die medizinische Beratung durch Ärzte. Der Leser/die Leserin sollte regelmäßig einen Arzt/eine Ärztin hinsichtlich Fragen zu seiner/ihrer Gesundheit und vor allem in Bezug auf Symptome, die eventuell einer ärztlichen Diagnose oder Behandlung bedürfen, konsultieren.\n\nDie Informationen in diesem Buch sind dazu gedacht, ein ordnungsgemäßes Training zu ergänzen, nicht aber zu ersetzen. Wie jeder andere Sport, der Geschwindigkeit, Ausrüstung, Gleichgewicht und Umweltfaktoren einbezieht, stellt dieser Sport ein gewisses Risiko dar. Die Autorin und Herausgeberin rät den Lesern dazu, die volle Verantwortung für die eigene Sicherheit zu übernehmen und die eigenen Grenzen zu beachten. Vor dem Ausüben der in diesem Buch beschriebenen Übungen solltest Du sicherstellen, dass Deine Ausrüstung in gutem Zustand ist, und Du solltest keine Risiken außerhalb Deines Erfahrungs- oder Trainingsniveaus, Deiner Fähigkeiten oder Deines Komfortbereichs eingehen.\nHintergrundillustrationen Urheberrecht © 2013 bei Shuttershock, Buchgestaltung und -produktion durch Anna Anderson Verfasst von Anna Anderson\nUrheberrecht © 2014 Instafemmefitness. Alle Rechte vorbehalten\n\nÜber mich",
        [
            "Fit in vier Wochen",
            "Deine Anleitung für eine reine Ernährung und ein gesünderes und glücklicheres Leben",
            "RECHTLICHE HINWEISE",
            "Ohne die ausdrückliche schriftliche Genehmigung der Eigentümerin von instafemmefitness, Anna Anderson, darf dieses E-Book weder teilweise noch in vollem Umfang reproduziert, gespeichert, kopiert oder auf irgendeine Weise übertragen werden.",
            "Wenn Du das E-Book auf einem öffentlich zugänglichen Computer ausdruckst, musst Du es nach dem Ausdrucken von dem Computer löschen.",
            "Jedes E-Book wird mit einem Benutzernamen und Transaktionsinformationen versehen.",
            "Verstöße gegen dieses Urheberrecht werden im vollen gesetzlichen Umfang geltend gemacht.",
            "Obgleich die Autorin und Herausgeberin alle Anstrengungen unternommen hat, sicherzustellen, dass die Informationen in diesem Buch zum Zeitpunkt der Drucklegung korrekt sind, übernimmt die Autorin und Herausgeberin keine Haftung für etwaige Verluste, Schäden oder Störungen, die durch Fehler oder Auslassungen in Folge von Fahrlässigkeit, zufälligen Umständen oder sonstigen Ursachen entstehen, und lehnt hiermit jedwede solche Haftung ab.",
            "Dieses Buch ist kein Ersatz für die medizinische Beratung durch Ärzte.",
            "Der Leser/die Leserin sollte regelmäßig einen Arzt/eine Ärztin hinsichtlich Fragen zu seiner/ihrer Gesundheit und vor allem in Bezug auf Symptome, die eventuell einer ärztlichen Diagnose oder Behandlung bedürfen, konsultieren.",
            "Die Informationen in diesem Buch sind dazu gedacht, ein ordnungsgemäßes Training zu ergänzen, nicht aber zu ersetzen.",
            "Wie jeder andere Sport, der Geschwindigkeit, Ausrüstung, Gleichgewicht und Umweltfaktoren einbezieht, stellt dieser Sport ein gewisses Risiko dar.",
            "Die Autorin und Herausgeberin rät den Lesern dazu, die volle Verantwortung für die eigene Sicherheit zu übernehmen und die eigenen Grenzen zu beachten.",
            "Vor dem Ausüben der in diesem Buch beschriebenen Übungen solltest Du sicherstellen, dass Deine Ausrüstung in gutem Zustand ist, und Du solltest keine Risiken außerhalb Deines Erfahrungs- oder Trainingsniveaus, Deiner Fähigkeiten oder Deines Komfortbereichs eingehen.",
            "Hintergrundillustrationen Urheberrecht © 2013 bei Shuttershock, Buchgestaltung und -produktion durch Anna Anderson Verfasst von Anna Anderson",
            "Urheberrecht © 2014 Instafemmefitness.",
            "Alle Rechte vorbehalten",
            "Über mich",
        ],
        marks=pytest.mark.xfail,
    ),
    pytest.param(
        "Es gibt jedoch einige Vorsichtsmaßnahmen, die Du ergreifen kannst, z. B. ist es sehr empfehlenswert, dass Du Dein Zuhause von allem Junkfood befreist. Ich persönlich kaufe kein Junkfood oder etwas, das nicht rein ist (ich traue mir da selbst nicht!). Ich finde jeden Vorwand, um das Junkfood zu essen, vor allem die Vorstellung, dass ich nicht mehr in Versuchung kommen werde, wenn ich es jetzt aufesse und es weg ist. Es ist schon komisch, was unser Verstand mitunter anstellt!",
        [
            "Es gibt jedoch einige Vorsichtsmaßnahmen, die Du ergreifen kannst, z. B. ist es sehr empfehlenswert, dass Du Dein Zuhause von allem Junkfood befreist.",
            "Ich persönlich kaufe kein Junkfood oder etwas, das nicht rein ist (ich traue mir da selbst nicht!).",
            "Ich finde jeden Vorwand, um das Junkfood zu essen, vor allem die Vorstellung, dass ich nicht mehr in Versuchung kommen werde, wenn ich es jetzt aufesse und es weg ist.",
            "Es ist schon komisch, was unser Verstand mitunter anstellt!",
        ],
        marks=pytest.mark.xfail,
    ),
    pytest.param(
        "Ob Sie in Hannover nur auf der Durchreise, für einen längeren Aufenthalt oder zum Besuch einer der zahlreichen Messen sind: Die Hauptstadt des Landes Niedersachsens hat viele Sehenswürdigkeiten und ist zu jeder Jahreszeit eine Reise Wert. Hannovers Ursprünge können bis zur römischen Kaiserzeit zurückverfolgt werden, und zwar durch Ausgrabungen von Tongefäßen aus dem 1. -3. Jahrhundert nach Christus, die an mehreren Stellen im Untergrund des Stadtzentrums durchgeführt wurden.",
        [
            "Ob Sie in Hannover nur auf der Durchreise, für einen längeren Aufenthalt oder zum Besuch einer der zahlreichen Messen sind: Die Hauptstadt des Landes Niedersachsens hat viele Sehenswürdigkeiten und ist zu jeder Jahreszeit eine Reise Wert.",
            "Hannovers Ursprünge können bis zur römischen Kaiserzeit zurückverfolgt werden, und zwar durch Ausgrabungen von Tongefäßen aus dem 1. -3. Jahrhundert nach Christus, die an mehreren Stellen im Untergrund des Stadtzentrums durchgeführt wurden.",
        ],
        marks=pytest.mark.xfail,
    ),
    pytest.param(
        "• 3. Seien Sie achtsam bei der Auswahl der Nahrungsmittel! \n• 4. Nehmen Sie zusätzlich Folsäurepräparate und essen Sie Fisch! \n• 5. Treiben Sie regelmäßig Sport! \n• 6. Beginnen Sie mit Übungen für die Beckenbodenmuskulatur! \n• 7. Reduzieren Sie Ihren Alkoholgenuss! \n",
        [
            "• 3. Seien Sie achtsam bei der Auswahl der Nahrungsmittel!",
            "• 4. Nehmen Sie zusätzlich Folsäurepräparate und essen Sie Fisch!",
            "• 5. Treiben Sie regelmäßig Sport!",
            "• 6. Beginnen Sie mit Übungen für die Beckenbodenmuskulatur!",
            "• 7. Reduzieren Sie Ihren Alkoholgenuss!",
        ],
        marks=pytest.mark.xfail,
    ),
    (
        "Was sind die Konsequenzen der Abstimmung vom 12. Juni?",
        ["Was sind die Konsequenzen der Abstimmung vom 12. Juni?"],
    ),
    (
        "Was pro Jahr10. Zudem pro Jahr um 0.3 %11. Der gängigen Theorie nach erfolgt der Anstieg.",
        [
            "Was pro Jahr10.",
            "Zudem pro Jahr um 0.3 %11.",
            "Der gängigen Theorie nach erfolgt der Anstieg.",
        ],
    ),
    ("s. vorherige Anmerkung.", ["s. vorherige Anmerkung."]),
    (
        "Mit Inkrafttreten des Mindestlohngesetzes (MiLoG) zum 01. Januar 2015 werden in Bezug auf den Einsatz von Leistungs.",
        [
            "Mit Inkrafttreten des Mindestlohngesetzes (MiLoG) zum 01. Januar 2015 werden in Bezug auf den Einsatz von Leistungs."
        ],
    ),
    pytest.param(
        "\n• einige Sorten Weichkäse  \n• rohes oder nicht ganz durchgebratenes Fleisch  \n• ungeputztes Gemüse und ungewaschener Salat  \n• nicht ganz durchgebratenes Hühnerfleisch, rohe oder nur weich gekochte Eier",
        [
            "\n",
            "• einige Sorten Weichkäse",
            "• rohes oder nicht ganz durchgebratenes Fleisch",
            "• ungeputztes Gemüse und ungewaschener Salat",
            "• nicht ganz durchgebratenes Hühnerfleisch, rohe oder nur weich gekochte Eier",
        ],
        marks=pytest.mark.xfail,
    ),
]


@pytest.mark.parametrize("text,expected_sentences", tests)
def test_segment(text, expected_sentences):
    assert list(segment("de", text)) == expected_sentences
