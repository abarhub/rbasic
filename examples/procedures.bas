REM --- Procedures (SUB / END SUB / CALL) ---

PRINT "=== Appel simple ==="
CALL Bonjour
CALL Bonjour

PRINT "=== Parametre entier ==="
CALL AfficheDouble(5)
CALL AfficheDouble(21)

PRINT "=== Parametre chaine ==="
CALL Saluer("Alice")
CALL Saluer("Bob")

PRINT "=== Plusieurs parametres ==="
CALL Rectangle(6, 4)
CALL Rectangle(10, 3)

PRINT "=== Variables locales ==="
X = 100
CALL ModifieX
PRINT "X apres CALL =", X

PRINT "=== SUB avec boucle ==="
CALL Compte(5)

PRINT "=== SUB imbriques ==="
CALL Externe

GOTO fin

REM --- Definitions des sous-programmes ---

SUB Bonjour
    PRINT "Bonjour !"
END SUB

SUB AfficheDouble(N)
    PRINT "Double de", N, "=", N * 2
END SUB

SUB Saluer(NOM$)
    PRINT "Salut, " + NOM$
END SUB

SUB Rectangle(L, H)
    PRINT "Aire", L, "x", H, "=", L * H
END SUB

SUB ModifieX
    X = 999
    PRINT "X dans SUB =", X
END SUB

SUB Compte(MAX)
    FOR I = 1 TO MAX
        PRINT I
    NEXT I
END SUB

SUB Externe
    PRINT "debut Externe"
    CALL Interne
    PRINT "fin Externe"
END SUB

SUB Interne
    PRINT "dans Interne"
END SUB

fin:
