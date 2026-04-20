REM --- STRING$, RND, RANDOMIZE, SLEEP, TIMER ---

PRINT "=== STRING$ ==="
PRINT STRING$(5, "A")
PRINT STRING$(3, 65)
PRINT STRING$(10, "-")
PRINT "|" + STRING$(4, " ") + "|"
S$ = "hello"
PRINT STRING$(LEN(S$), "*")

PRINT "=== TIMER ==="
T = TIMER
PRINT "Secondes depuis epoch :", INT(T)

PRINT "=== RANDOMIZE et RND ==="
RANDOMIZE TIMER
FOR I = 1 TO 5
    PRINT INT(RND * 100)
NEXT I

PRINT "=== Sequence reproductible ==="
RANDOMIZE 42
FOR I = 1 TO 3
    PRINT RND
NEXT I

PRINT "=== SLEEP ==="
PRINT "avant"
SLEEP 0
PRINT "apres (SLEEP 0)"

PRINT "=== De 1 a 10 au hasard ==="
RANDOMIZE TIMER
FOR I = 1 TO 5
    N = INT(RND * 10) + 1
    PRINT N
NEXT I
