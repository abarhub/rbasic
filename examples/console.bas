REM --- Démonstration des commandes console ---
REM    SCREEN, WIDTH, COLOR, LOCATE, CLS, BEEP, INKEY$, CSRLIN, POS

SCREEN 0        ' Mode texte (no-op)
WIDTH 80        ' Largeur 80 colonnes (no-op)

REM === CLS et positionnement ===
CLS
LOCATE 1, 1
COLOR 15, 1     ' Texte blanc sur fond bleu
PRINT "=== Démo Console rbasic ==="
COLOR 7, 0      ' Retour à la normale

REM === Palette de couleurs ===
LOCATE 3, 1
PRINT "Palette de couleurs :"
FOR C = 0 TO 15
    COLOR C, 0
    LOCATE 4, C * 3 + 1
    PRINT C
NEXT C
COLOR 7, 0

REM === Positionnement précis ===
LOCATE 6, 1
COLOR 10
PRINT "Bonjour depuis la ligne 6 !"
COLOR 7

LOCATE 7, 20
COLOR 12
PRINT "Colonne 20"
COLOR 7

REM === BEEP ===
LOCATE 9, 1
PRINT "Bip !"
BEEP

REM === CSRLIN et POS ===
LOCATE 11, 1
PRINT "Position curseur : ligne =", CSRLIN, " colonne =", POS(0)

REM === INKEY$ — attente d'une touche ===
LOCATE 13, 1
COLOR 14
PRINT "Appuyez sur une touche (q pour quitter)..."
COLOR 7

DO
    K$ = INKEY$
    IF K$ <> "" THEN
        LOCATE 14, 1
        PRINT "Touche appuyée : ["; K$; "]"
        IF K$ = "q" OR K$ = "Q" THEN
            GOTO fin
        END IF
    END IF
LOOP

fin:
LOCATE 16, 1
COLOR 10
PRINT "Au revoir !"
COLOR 7, 0
