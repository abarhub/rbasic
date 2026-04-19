REM --- Labels et GOTO ---
PRINT "=== GOTO ==="
GOTO suite
PRINT "cette ligne est sautee"
suite:
PRINT "apres le GOTO"

REM --- IF / THEN / ELSE ---
PRINT "=== IF / THEN / ELSE ==="
X = 7
IF X > 5 THEN PRINT "X est grand"
IF X < 5 THEN PRINT "X est petit" ELSE PRINT "X >= 5"

REM --- IF avec affectation ---
NOTE$ = ""
IF X = 7 THEN NOTE$ = "exactement 7"
PRINT "Note : " + NOTE$

REM --- GOTO numerote ---
PRINT "=== GOTO avec numeros ==="
10 GOTO 30
20 PRINT "ligne 20 sautee"
30 PRINT "ligne 30 atteinte"

REM --- FOR / NEXT ---
PRINT "=== FOR / NEXT ==="
FOR I = 1 TO 5
PRINT "I =", I
NEXT I

REM --- FOR avec STEP ---
PRINT "=== FOR avec STEP 2 ==="
FOR J = 0 TO 10 STEP 2
PRINT J
NEXT J

REM --- FOR compte a rebours ---
PRINT "=== Compte a rebours ==="
FOR K = 5 TO 1 STEP -1
PRINT K
NEXT K
PRINT "Decollage !"

REM --- FOR cumul ---
PRINT "=== Somme 1..10 ==="
S = 0
FOR N = 1 TO 10
S = S + N
NEXT N
PRINT "Somme =", S

REM --- WHILE / WEND ---
PRINT "=== WHILE / WEND ==="
C = 1
WHILE C <= 5
PRINT "C =", C
C = C + 1
WEND

REM --- WHILE condition fausse au depart ---
PRINT "=== WHILE jamais execute ==="
WHILE 0
PRINT "ne doit pas s afficher"
WEND
PRINT "WHILE 0 ignore"

REM --- Factorielle avec WHILE ---
PRINT "=== Factorielle de 6 ==="
N = 6
F = 1
WHILE N > 1
F = F * N
N = N - 1
WEND
PRINT "6! =", F
