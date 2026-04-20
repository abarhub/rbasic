REM --- IF multiligne, DO/LOOP, DECLARE SUB ---

DECLARE SUB AfficheNote(N)

PRINT "=== IF multiligne ==="
X = 7
IF X > 10 THEN
    PRINT "grand"
ELSEIF X > 5 THEN
    PRINT "moyen"
ELSE
    PRINT "petit"
END IF

PRINT "=== IF imbrique ==="
A = 1
B = 2
IF A = 1 THEN
    IF B = 2 THEN
        PRINT "A=1 et B=2"
    ELSE
        PRINT "A=1 mais B<>2"
    END IF
END IF

PRINT "=== DO WHILE ==="
I = 1
DO WHILE I <= 5
    PRINT I
    I = I + 1
LOOP

PRINT "=== DO UNTIL ==="
I = 5
DO UNTIL I <= 0
    PRINT I
    I = I - 1
LOOP

PRINT "=== DO ... LOOP WHILE (post-condition) ==="
I = 1
DO
    PRINT I
    I = I + 1
LOOP WHILE I <= 3

PRINT "=== DO ... LOOP UNTIL (au moins une fois) ==="
I = 10
DO
    PRINT "au moins une fois :", I
    I = I + 1
LOOP UNTIL I < 5

PRINT "=== DECLARE SUB ==="
CALL AfficheNote(18)
CALL AfficheNote(10)
CALL AfficheNote(7)

SUB AfficheNote(N)
    IF N >= 16 THEN
        PRINT N, ": Tres bien"
    ELSEIF N >= 10 THEN
        PRINT N, ": Passable"
    ELSE
        PRINT N, ": Insuffisant"
    END IF
END SUB
