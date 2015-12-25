# ovisbp
Hallo liebe mit-rostende Gemeinde!

Eine neue Challenge!

Es soll, zumindest Ansatzweise, kooperativ werden. Dazu gleich mehr.

Das Ziel ist ein kleines 2D-Spiel. Dabei ist die Ausgabe am Ende komplett freigestellt, es ist so konzipiert dass es auch ohne Probleme möglich ist es einfach auf dem Terminal auszugeben (denke ich zumindest), ansonsten sind der Phantasie was die Grafik angeht natürlich KEINE GRENZEN gesetzt :)

Das Spielprinzip ist ganz simpel. Es gibt ein Level, und einen Spieler. Der Spieler startet an einem punkt in dem level, und hat als Ziel (erstmal) an einen anderen punkt zu gelangen. Dabei kann er sich bewegen (Rechts, Links), und springen ("Schwerkraft": Geht der spieler über eine Kante, "fällt" er nach unten bis wieder Boden da ist).
Schaut euch dazu am Besten mal das Sketch im Anhang an.

Das Level ist also eigentlich ein x*y grid und besteht aus x*y Fields. Ein Field kann entweder leer sein (der Spieler kann sich da durch bewegen), oder ein Block sein (der Spieler kann sich NICHT da durch bewegen, sondern "nur" da drauf stehen bzw. laufen.

Die Aufgabe ist also, a) ein (oder mehrere) Level zu bauen, mit Start und Endpunkt, und Blöcken und Wegen dazwischen, und b) einen Spieler zu entwerfen, der sich bewegen und springen kann, um vom Start zum Ziel zu gelangen. Denkbar ist auch, Blöcke zerstörbar zu machen, sodass ein Spieler irgendwelche Blöcke erst kaputt machen muss, bevor er ans Ziel kommt.


Wo ist jetzt die "Kooperation"? Ich dachte mir es wäre vielleicht cool, wenn man die Level untereinander austauschen kann, sodass ich mit meinem Spieler (bzw. meiner Spiellogik und meiner Implementierung der Darstellung des Spiels) das Level eines anderen "spielen" kann. Der erste Gedanke war die Level in einer art xml oder json oder so abzuspeichern, aber das wäre ja langweilig, und hat auch nichts mit Rust zu tun.

Was ich mir also gedacht habe ist folgendes: Bei der Implementierung des Levels halten wir uns alle an ein einheitliches Interface, für das ich hier schonmal eine erste Version erstellt habe: https://github.com/lanice/ovisbp 
In der src/lib.rs findet man eigentlich nur eine Art Spezifikation für ein Level, und dessen Fields bzw. Blocks, in form von traits. Der Sinn ist also dass unser level Typ das trait Level implementiert, und andersrum, dass wir, wenn wir unseren Spieler implementieren, er nur auf das Interface von Level zugreift. Dadurch kann zb ich ganz einfach (einfach zumindest laut Plan ;) ) am Ende zb das Projekt von Moritz (der es ja auch immer brav auf github hat) als dependency angeben, und statt meinem eigenen Level das von Moritz verwenden, und trotzdem mit meiner Spielimplementierung spielen.

Natürlich ist dieses Interface nicht endgültig festgeschrieben. Ich hab mir da zwar schon mittlerweile einige Gedanken zu gemacht, aber es ja auch noch nicht selber implementiert. Falls also beim Programmieren jemand merkt, da fehlt irgendwie noch eine Funktion, dann würde ich sagen diskutieren wir das vielleicht kurz (dazu gibt es ja Issues), und dann fügt derjenige die eben hinzu, also erweitert das Interface. Oder auch wenn es noch eine super coole Idee dazu gibt. Oder natürlich, ganz wichtig, wenn jemand merkt dass da etwas einfach fehlerhaft ist, also zb irgendwo ein Typ falsch angegeben ist, eine Referenz ist wo keine hingehört, und und und. Ich habe das Interface heute nur einmal ganz kurz getestet, es kann ohne Weiteres sein dass sich da noch der ein oder andere Fehler eingeschlichen hat.

Das heißt im Klartext: Wir arbeiten eventuell zusammen noch an dem Interface ovisbp. Dann bzw. parallel erstellt jeder eine Implementierung für ein (oder mehrere) Level, sowie eine Spiellogik die einen Spieler beeinhaltet, der zum Ziel hat, von Level::start_position() nach Level::end_position() zu gelangen. Wie er das macht ist im Prinzip egal, ich meine theoretisch könntet ihr ja auch teleport funktionen für den Spieler implementieren. Ich würde jedoch sagen es geht erstmal nur um Bewegung (links, rechts, sprung), und vielleicht auch Blöcke kaputt machen/schießen/furzen.

Und denkt bei der Spiellogik dran: Am besten werden nur die Funktionen aus den Traits genutzt (im Zweifel erweitern), und keine eigenen "lokal" zum struct hinzugefügten, da man dann die Level-implementierungen austauschen kann (denke ich zumindest)!

Ach ja zur Erklärung des Namens, der setzt sich zusammen aus 'ovis' (lat. für 'Schaf'), und 'bp' (für 'blueprint', von wegen ist ja nur das interface).

Falls es Fragen gibt, immer her damit ;D



PS: Bonusaufgabe mini-KI: Das Programm den Spieler steuern lassen (hands-free, yeyyyy)

PPS: Und immer dran denken: Get Schwi... *hust* Rusty!
