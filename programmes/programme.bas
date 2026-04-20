$Debug
' programme en qbasic base sur le magazine science et vie junior no 35 (version corrigee)
' affichage sur 40 colonnes
' TS = tableau de tetris (ligne,colonne) (24 lignes et 11 colonnes)
' les colonnes 1 et 11 et la ligne 24 correspondent aux bords et ne contiennent pas de cube
' SC = les caracteres de l'ecran avec comme valeur la couleur a afficher
10 Screen 0:
Width 40:
KEY Off:
Cls:
Randomize Timer
20 R$ = "2122232412223242212223241222324231223223122223333122322312222333212232331322322321223233132232232232233322322333223223332232233321222333122232131121222312223132212213231213233311211213122232331222323212232231221223221122223"
' initialisation de A$
30 For i = 1 To 3:
    For j = 0 To 3:
        For K = 1 To 4:
            A$(i, j, K) = Mid$(R$, 32 * i - 31 + 2 + K - 2 + 8 * j, 2):
Next K, j, i
' SC = score
40 SC = 0:
S$ = Chr$(219):
Dim B$(23), TS(24, 11):
For i = 1 To 23:
    B$(i) = Space$(9):
Next i
' affichage des bords verticaux
50 Color 11:
For i = 3 To 23:
    Locate i, 10:
    Print S$;:
    TS(i, 1) = 11:
    Locate i, 20:
    Print S$;:
    TS(i, 11) = 11:
Next i
' affichage du bord horizontal
60 Locate 24, 10:
Print String$(11, S$);:
For K = 1 To 11:
    TS(24, K) = 11:
Next K
' affichage du score
70 Color 7:
XA = 0:
XO = 1:
YO = 13:
K = 1:
H = 8:
S = Int(Rnd * 7 + 1):
Locate 1, 1:
Print "SCORE "; SC
' debut de la boucle principale
80 Color S:
R$ = "":
A$ = S$:
PP = S:
GoSub 150
90 For i = 1 To 40
    R$ = Right$(InKey$, 1)
    ' GOTO ou GOSUB ?
    If R$ <> "" Then GoTo 100
Next i
100 A$ = " ":
PP = 0:
GoSub 150:
XI = XO:
YI = YO:
KI = K
110 T = T + 1:
If (T Mod H = 0 Or R$ = " ") And R$ <> "1" And R$ <> "3" Then
    XA = XO:
    XO = XO + 1
End If
120 K = (K - 3 * (R$ = "2") - (R$ = "5")) Mod 4:
YO = YO + (1 And R$ = "3") - (1 And R$ = "1")
130 P = 0:
For i = 1 To 4:
    posx = XO + Val(Right$(A$(S, K, i), 1))
    posy = YO - 9 + Val(Left$(A$(S, K, i), 1))
    If posx >= 0 And posx < 24 And posy >= 0 And posy < 11 Then
        ' verifier si c'est bien dans P qu'il faut mettre ce calcul
        P = P + TS(posx, posy):
    End If
Next i:
If P <> 0 Then
    XO = XI:
    YO = YI:
    K = KI
endif
140 If XA = XO Then
    A$ = S$:
    PP = S:
    TT = 1:
    GoSub 150:
    P = 1:
    GoSub 180:
    If XO <> 1 Then
        GoTo 70
    Else
        ' fin du jeux
        Print "C'est fini":
        End
    End If
Else
    ' fin de la boucle principale
    'delay 10000
    'Sleep 1
    GoTo 80
End If
' fonction affichage sur l'ecran
' parametres :
' $A = chaine a afficher a l'ecran
' PP = caractere a mettre dans le tableau TS
' XO =
' YO =
150 For i = 1 To 4:
    TX = XO + Val(Right$(A$(S, K, i), 1)):
    TY = YO + Val(Left$(A$(S, K, i), 1)):
    If TX >= 0 And TX < 24 And TY - 9 >= 0 And TY - 9 < 11 Then
        Locate TX, TY:
        Print A$;:

        TS(TX, TY - 9) = PP

        160 If TT = 1 Then
            Mid$(B$(TX), TY - 10) = A$
        End If
    End If
170 Next i:
TT = 0:
Return
' fonction d'affichage du cadre
180 P = 3 * P:
For i = 23 To 1 Step -1
    190 If B$(i) = String$(9, S$) Then
        For j = i To 2 Step -1:
            B$(j) = B$(j - 1):
            For L = 1 To 11:
                TS(j, L) = TS(j - 1, L):
        Next L, j:
        B$(1) = Space$(9):
        For L = 2 To 10:
            TS(1, L) = 0:
        Next L:
        TS(3, 1) = 11:
        TS(3, 11) = 11:
        Print Chr$(7);:
        SC = SC + P:
        GoTo 180
    End If
    200 If P <> 3 Then
        For L = 1 To 11:
            Locate i, 9 + L:
            Color TS(i, L):
            Print S$:
        Next
    End If
210 Next i:
Return

' delay
DECLARE SUB delay (duration!)
Sub delay (duration As Single)
    tim = Timer
    Do
    Loop Until (Timer - tim + 86400) - (Int((Timer - tim + 86400) / 86400) * 86400) > duration
End Sub

