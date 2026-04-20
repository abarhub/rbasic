' programme en qbasic base sur le magazine science et vie junior no 35 (version initiale)
10 Screen 0: Width 40: KEY Off: Cls: Randomize Timer
20 R$ = "2122232412223242212223241222324231223223122223333122322312222333212232331322322321223233132232232232233322322333223223332232233321222333122232131121222312223132212213231213233311211213122232331222323212232231221223221122223"
30 For i = 1 To 3: For j = 0 To 3: For K = 1 To 4: A$(i, j, K) = Mid$(R$, 32 * i - 31 + 2 + K - 2 + 8 * j, 2): Next K, j, i
' TS=T$ ?
40 SC = 0: S$ = Chr$(219): Dim B$(23), TS(24, 11): For i = 1 To 23: B$(i) = Space$(9): Next i
50 Color 11: For i = 3 To 23: Locate i, 10: Print S$;: TS(i, 1) = 11: Locate i, 20: Print S$;: TS(i, 11) = 11: Next i
60 Locate 24, 10: Print String$(11, S$);: For K = 1 To 11: TS(24, K) = 11: Next K
70 Color 7: XA = 0: XO = 1: YO = 13: K = 1: H = 8: S = Int(Rnd * 7 + 1): Locate 1, 1: Print "SCORE "; SC
80 Color S: R$ = "": A$ = S$: PP = S: GoSub 150
90 For i = 1 To 40
    R$ = Right$(InKey$, 1)
    If R$ <> "" Then GoSub 100
Next i
100 A$ = " ": PP = 0: GoSub 150: XI = XO: YI = YO: KI = K
110 T = T + 1: If (T Mod H = 0 Or R$ = " ") And R$ <> "1" And R$ <> "3" Then XA = XO: XO = XO + 1
120 K = (K - 3 * (R$ = "2") - (R$ = "5")) Mod 4: YO = YO + (1 And R$ = "3") - (1 And R$ = "1")
130 P = 0:
For i = 1 To 4:
    ' voir quoi mettre a la place de GG
    GG = P + TS(XO + Val(Right$(A$(S, K, i), 1)), YO - 9 + Val(Left$(A$(S, K, i), 1))):
Next i:
If P <> 0 Then XO = XI: YO = YI: K = KI
140 If XA = XO Then A$ = S$: PP = S: TT = 1: GoSub 150: P = 1: GoSub 180: If XO <> 1 Then GoTo 70 Else Print "C'est fini": End Else GoTo 80
150 For i = 1 To 4: TX = XO + Val(Right$(A$(S, K, i), 1)): TY = YO + Val(Left$(A$(S, K, i), 1)): Locate TX, TY: Print A$;: TS(TX, TY - 9) = PP
    160 If TT = 1 Then Mid$(B$(TX), TY - 10) = A$
170 Next i: TT = 0: Return
180 P = 3 * P: For i = 23 To 1 Step -1
    190 If B$(i) = String$(9, S$) Then For j = i To 2 Step -1: B$(j) = B$(j - 1): For L = 1 To 11: TS(j, L) = TS(j - 1, L): Next L, j: B$(1) = Space$(9): For L = 2 To 10: TS(1, L) = 0: Next L: TS(3, 1) = 11: TS(3, 11) = 11: Print Chr$(7);: SC = SC + P: GoTo 180
    200 If P <> 3 Then For L = 1 To 11: Locate i, 9 + L: Color TS(i, L): Print S$: Next
210 Next i: Return
