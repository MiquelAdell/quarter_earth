# Guió de l’assemblea: guia per a la persona facilitadora

> Traducció de run-sheet.md, pendent de revisió per una persona de parla catalana (vegeu content/README.md). Si hi ha diferències, mana la versió anglesa.

Aquest guió cobreix una assemblea de Quarter Earth de tres hores per a 12–24 persones repartides en quatre taules. Cal una persona facilitadora, un ordinador portàtil per fer anar el model, un projector i una paret. Tingues-lo obert al portàtil durant la sessió.

El material en paper és a [`print/README.md`](../print/README.md). Les regles segueixen `openspec/changes/assembly-workshop/specs/assembly-session/spec.md`, a la carpeta personal del projecte. Si una regla d’aquí contradiu aquesta especificació, mana l’especificació: primer es modifica l’especificació i després aquest guió.

> Les dades de les cartes, els requisits, el conflicte ocult del carbó, els temps, els valors inicials i els anys de cada ronda vénen de `references/assembly/model-baseline.md` de W1 (commit 45277e6e) i de la fila del 2022 de `fixtures/a_no_action/ozone_appended.json`. Els TODO pendents són a Qüestions obertes.

## El rellotge: 180 minuts

| Inici | Fi | Min | Bloc | Bloc de l’especificació |
|---|---|---|---|---|
| 0:00 | 0:15 | 15 | Asseure les taules, llegir el planeta inicial, mostrar una carta en un món de prova i reiniciar | Obertura i pràctica **30** |
| 0:15 | 0:30 | 15 | Compromisos i preguntes, acord de pràctica entre dues taules, prediccions personals | ″ |
| 0:30 | 0:50 | 20 | Ronda 1 (baralla A) | Rondes 1–2 **40** |
| 0:50 | 1:10 | 20 | Ronda 2 | ″ |
| 1:10 | 1:20 | 10 | Pausa. Deixar a la vista la cronologia i les propostes | Pausa **10** |
| 1:20 | 1:40 | 20 | Ronda 3. Canvia qui fa de portaveu, cartes que s’acaben d’habilitar | Rondes 3–4 **40** |
| 1:40 | 2:00 | 20 | Ronda 4 | ″ |
| 2:00 | 2:15 | 15 | Ronda 5. Les taules revisen propostes anteriors o renoven prioritats | Rondes 5–6 **30** |
| 2:15 | 2:30 | 15 | Ronda 6. L’última votació s’ha de fer a temps | ″ |
| 2:30 | 2:45 | 15 | Repetició d’una decisió discutida | Repetició **15** |
| 2:45 | 3:00 | 15 | Balanç | Balanç **15** |

Comprovació: 30 + 40 + 10 + 40 + 30 + 15 + 15 = **180**.

Anys de cada ronda (model-baseline.md §3). Les taules planifiquen el primer any de cada ronda i la partida acaba el 2052.

| Ronda | Any de planificació | Anys que cobreix |
|---|---|---|
| 1 | 2022 | 2022 → 2027 |
| 2 | 2027 | 2027 → 2032 |
| 3 | 2032 | 2032 → 2037 |
| 4 | 2037 | 2037 → 2042 |
| 5 | 2042 | 2042 → 2047 |
| 6 | 2047 | 2047 → 2052 |

**Les cartes triguen uns quatre anys a entrar en funcionament.** Una carta aprovada l’any de planificació Y és en construcció durant els tres anys següents. S’activa l’any Y+4 i canvia la producció per primer cop l’últim any de la ronda, Y+4 → Y+5. Per això, la fase de Resultats just després d’una votació mostra **com a molt un any d’efecte**, i l’efecte complet apareix a les rondes següents. Digues-ho a la sala a les 0:15 i un altre cop abans dels primers Resultats, i tingues-ho en compte en valorar les prediccions.

### Dins d’una ronda

| Fase | Ronda de 20 min (1–4) | Ronda de 15 min (5–6) |
|---|---|---|
| Lectura de l’any | 0–2 (2) | 0–2 (2) |
| Preparar una proposta | 2–6 (4) | 2–5 (3) |
| Negociació | 6–12 (6) | 5–9 (4) |
| Decisió | 12–15 (3) | 9–11 (2) |
| Resultats | 15–20 (5) | 11–15 (4) |
| **Total** | **20** | **15** |

Els retards es descompten de **Resultats**. La Decisió no es mou mai. Una segona volta o un paquet no vàlid consumeixen al voltant d’un minut de Resultats; quan passi, salta’t la pregunta sobre sorpreses a la Lectura següent en lloc d’anar amb retard.

## Abans que arribi la gent

- [ ] El material de deu fulls és imprès i retallat (vegeu `print/README.md`). A cada taula hi ha una baralla A, un mandat i un bolígraf.
- [ ] Els dos tapets de coalició són en fundes A4 sobre la **taula central**, al costat de la persona operadora, amb dos retoladors esborrables i un drap.
- [ ] Cada taula té set fulls de paquet: un per a la pràctica i un per ronda. Cada persona té un joc de targetes de vot (A / B / =) i un full de predicció.
- [ ] Cronologia mural: els fulls de les rondes 1–6 i el de repetició estan enganxats en ordre, amb espai al costat per al compromís i la pregunta de cada taula.
- [ ] El portàtil té carregat el món fixat. S’ha provat de restaurar i repetir en aquest mateix portàtil.
  - Per a l’assaig (W4), fes servir el mode taller existent amb `WORKSHOP=1 just run` o `?workshop=1`. Carrega el món des del menú principal. No facis servir `DEBUG_VIEW=Plan`. Les notes d’arrencada de M1 són a `references/simplification/m1-run-sheet.md`.
  - TODO W5/W6: el punt d’entrada de l’assemblea i les seves vistes Planeta / Votació / Informe substitueixen aquest pas.
- [ ] La llista de cartes disponibles és al full de la persona operadora. A la ronda 1 hi ha disponibles les vuit cartes de la baralla A. Les cartes amb requisit i el conflicte del carbó són a la comprovació de la persona operadora, més avall.

## Taules i mandats

| Codi | Taula | Respon de | Pregunta que pot portar |
|---|---|---|---|
| AT | Alimentació i terra | Alimentar la població durant la transició | D’on sortirà la terra? |
| EI | Energia i indústria | Mantenir la producció mentre es retallen emissions | Què substitueix el subministrament que retirem? |
| NV | Nivell de vida | Defensar el benestar diari i l’accés als béns bàsics | Qui canvia primer el seu consum? |
| MV | Món viu | Deixar hàbitats i biodiversitat en condicions de recuperar-se | Quina protecció sobreviu al pròxim pressupost? |

Totes les taules tenen les mateixes cartes i una responsabilitat diferent. Ningú no fa de lobby obstruccionista ni parla en nom d’una comunitat afectada.

## Rols rotatius

Cada taula cobreix aquests rols a cada ronda i apunta els noms a la graella de rols del seu full de mandat.

| Rol | Què fa |
|---|---|
| **Enllaç** | Porta el full i les cartes de la taula a altres taules durant la Negociació i torna amb les ofertes |
| **Registre** | Omple el full de paquet, els costos i les dues prediccions. Signa un tapet en nom de la taula quan la taula hi està d’acord |
| **Control del temps** | Vigila el rellotge projectat. Porta el tapet a la taula central **tan bon punt està signat**, i mai després del final de la Negociació. Compta les mans de la taula si cal |
| **Portaveu** | Llegeix l’any en veu alta a la Lectura. Les dues taules promotores d’un paquet trien **una** de les seves dues persones portaveus per defensar-lo en 30 segons a la Decisió |

La taula decideix qui fa cada rol, amb aquests límits:

- **Es rota a cada ronda.** Cada rol passa a la persona de l’esquerra.
- **L’enllaç rota del tot.** Ningú no repeteix com a enllaç fins que totes les persones de la taula ho hagin estat una vegada.
- **L’enllaç torna abans de signar.** L’enllaç ha de ser de tornada a la seva taula abans que aquesta signi cap tapet, i el tapet té una casella per marcar-ho. Un enllaç no pot comprometre la seva taula des d’una altra taula.
- **Les taules petites acumulen rols.** En una taula de tres, Control del temps i Portaveu van junts. En una taula de cinc o sis, les persones que sobren no tenen rol fix: reben visites i comproven costos.
- **Les aportacions per escrit compten.** Qualsevol pot escriure una esmena o una pregunta al dors del full en lloc de dir-la en veu alta, i qui porta el Registre la llegeix. Ofereix-ho explícitament a la ronda 1 perquè les veus més ràpides no dirigeixin totes les taules.

## Obertura i pràctica (0:00–0:30)

| Rellotge | Què cal fer |
|---|---|
| 0:00–0:05 | Asseu la gent en quatre taules de tres a sis persones. Reparteix els mandats. Es trien els rols de la pràctica. |
| 0:05–0:10 | Projecta el planeta inicial i llegiu-lo junts: any, emissions, temperatura, pressió d’extinció, satisfacció i l’indicador de terra. Cada taula llegeix en veu alta el seu requadre de dades. Són els valors del 2022 de la simulació sense mesures de W1. |
| 0:10–0:15 | **Mostra una carta** en una còpia de prova del món. Ensenya una carta sobre un tapet, el seu cost davant del pressupost de 30 punts, la introducció al portàtil, l’avanç i l’informe. Després **reinicia** al món inicial davant de la sala, perquè tothom vegi que no s’ha conservat res. |
| 0:15–0:20 | Cada taula escriu el seu **compromís públic**: un indicador, una direcció, un objectiu i un any. També escriu **una pregunta que el model no pot respondre**. Qui porta el Registre copia totes dues coses a la paret. |
| 0:20–0:27 | **Acord de pràctica.** AT treballa amb MV, i EI amb NV. Cada parella munta un paquet sobre un tapet, omple el pressupost i les comprovacions, escriu dues prediccions i signa. No es vota ni s’introdueix res. Després s’esborren els tapets. |
| 0:27–0:30 | Cada persona escriu un **full de predicció personal** i el guarda fins al balanç. |

Digues això a les 0:15: «Cada paquet té 30 punts, que no s’acumulen d’una ronda a l’altra. Un paquet necessita la signatura de dues taules. Cada taula pot signar un paquet per ronda, així que com a molt arriben dos paquets a votació. Signar no obliga ningú a votar d’una manera. Una carta triga uns quatre anys a entrar en funcionament, així que gairebé tot el que aproveu es notarà en rondes posteriors».

## Desenvolupament de cada ronda (rondes 1–6)

### 1. Lectura de l’any (2 min)

- La persona operadora projecta la vista **Planeta**. La persona portaveu de cada taula llegeix el titular en veu alta.
- Pregunta a una taula: «Què us va sorprendre a la ronda anterior?». No ho facis a la ronda 1 ni després d’una ronda que s’hagi allargat.
- La persona operadora anuncia les **cartes disponibles**: les que ja tenen els requisits en vigor, menys les ja aprovades. Les taules giren de cara avall les que no estan disponibles.
  - **Una carta només es pot aprovar una vegada.** El motor només accepta una carta mentre és `Inactive`. Un cop aprovada passa a `Building` i després a `Active`, així que no es pot tornar a aprovar (model-baseline.md §2, §5.1). És una regla del motor, no de la casa.
  - Cap carta de la baralla A no té requisits.
  - Una carta amb requisit es pot aprovar l’**any de planificació següent a l’aprovació de la carta que l’habilita**, és a dir, a la ronda següent. Requisits a la baralla completa: Agricultura regenerativa → Carn cultivada. Impuls solar, Impuls eòlic o Expansió nuclear → Electrificació massiva. Restaurar i protegir els ecosistemes → Gestió de la radiació solar (GRS).

### 2. Preparar una proposta (4 min, o 3 a les rondes 5–6)

- Primer, **30 segons de silenci**. Cada persona tria en privat una prioritat tocant una carta.
- Qui porta el Registre omple un **full de paquet** amb 1–3 cartes disponibles **diferents** que costin **30 punts o menys**, més **dues prediccions de direcció**: un indicador i ↑ o ↓.
- Les mesures que ja són en vigor continuen funcionant amb les seves pròpies regles. El pressupost només compra decisions noves. Els punts que no es gasten **es perden** al final de la ronda.
- A les rondes 5–6, les taules poden tornar a presentar o revisar un full anterior en lloc de començar de zero.

### 3. Negociació (6 min, o 4 a les rondes 5–6)

- L’enllaç porta el full, amb les seves cartes, a una altra taula. La resta rep visites, comprova costos o prepara esmenes. Les taules poden fer la seva pregunta del mandat als enllaços que les visiten.
- **Per arribar a votació**, dues taules es posen d’acord en un paquet. El seu Control del temps recull un tapet lliure a la taula central. Les cartes van a les caselles i els costos a sota. Se suma el total, s’ombreja **una casella de l’indicador per cada bloc de 5 punts** i es marquen les sis comprovacions.
- **Esmena.** Una carta substituïda al tapet porta marcada la casella «Substituïda», juntament amb quina carta **substitueix** i quina taula la va **proposar**. El que identifica l’esmena és la marca escrita. Mai no es fa servir el color per a això.
- **Signatura.** Cada enllaç torna a la seva taula i marca la casella. Qui porta el Registre signa per la taula, i les dues prediccions s’escriuen al tapet.
  - **Qualsevol canvi després d’una signatura anul·la les dues signatures**, i les dues taules tornen a signar.
- **Límits.** Cada taula signa **un tapet per ronda**. Hi ha dos tapets, així que com a molt arriben dos paquets a votació. Si signen les quatre taules, cadascuna promou exactament un paquet.
- **Lliurament.** El Control del temps porta el tapet a la taula central **tan bon punt les dues taules han signat**, perquè la persona operadora el comprovi i l’introdueixi mentre continua la Negociació. El final de la Negociació és l’últim moment per lliurar-lo, no el començament de la introducció. Un tapet amb una sola signatura en acabar el termini no arriba a votació.

### Comprovació de la persona operadora (durant la Negociació, a mesura que arriben els tapets)

Introdueix cada tapet a la vista de facilitació tan bon punt arribi. Fins que W6 construeixi aquesta vista, per exemple a l’assaig W4, comprova a mà les sis caselles del tapet i aprova les cartes a la interfície del taller existent després de la votació. Si es rebutja, llegeix el motiu en veu alta. Les taules promotores el corregeixen **a la taula central**, fent servir com a màxim un minut de Resultats.

| No és vàlid perquè | Què veu o comprova la persona operadora | La taula ho corregeix |
|---|---|---|
| Supera el pressupost (total > 30) | La suma dels costos de les caselles, que coincideix amb els blocs de l’indicador | Traient o canviant una carta fins que el total sigui 30 o menys |
| Carta repetida | El mateix ID de carta en dues caselles | Traient la repetida i, si vol, afegint-hi una altra carta diferent |
| Carta ja aprovada | La carta es va aprovar en una ronda anterior. El motor només accepta cartes que continuen `Inactive` | Canviant-la per una carta disponible |
| Requisit encara no aprovat | La línia «Requereix» de la carta davant de les cartes aprovades en rondes **anteriors**. Una carta habilitadora aprovada en aquesta ronda encara no compta. Cap carta de la baralla A no té requisits (vegeu els requisits a Lectura de l’any) | Canviant la carta o traient-la |
| Exclusió declarada | No n’hi ha cap a la baralla actual | — |
| **Avís, no rebuig: conflicte del carbó** | **Eliminar el carbó** juntament amb **Impuls solar** (o Expansió nuclear), en el mateix paquet o en rondes diferents. El carbó comença amb el 35 % de l’electricitat, i això no basta per als dos canvis. El motor accepta totes dues, cobra totes dues i les mostra com a actives, però la que s’aplica en segon lloc no fa res (model-baseline.md §5.1). La línia «No es combina amb» de totes dues cartes ho indica | Llegeix l’avís en veu alta. La taula es queda amb una de les dues o paga, sabent-ho, per una carta que no farà res |
| Taula promotora repetida (una taula als dos tapets) | Els codis de taula dels dos tapets | La taula tria un tapet. L’altre necessita la signatura d’una altra segona taula o no arriba a votació |
| Menys de dues taules diferents | Els codis de taula | Buscant una segona taula; si no, el paquet no arriba a votació |
| Més de tres cartes o cap | El nombre de caselles ocupades | Deixant-lo en 1–3 |

Si en començar la Decisió el paquet continua sense ser vàlid, **no va a votació** en aquella ronda. Qualsevol canvi obliga les dues taules a tornar a signar, igual que durant la Negociació.

### 4. Decisió (3 min, o 2 a les rondes 5–6)

- La vista **Votació** mostra cada paquet amb les seves cartes, costos, taules promotores i prediccions. Les prediccions queden fixades **abans** de votar i la persona operadora ja les ha introduïdes.
- Cada **paquet** té **30 segons**, amb una sola veu per paquet. Això suma 1 minut de defenses i deixa 1–2 minuts per votar i comptar en tots dos tipus de ronda.
- **Votació.** Cada participant té un vot i vota a títol individual. Compte enrere: «3, 2, 1, amunt!». Tothom aixeca **una** targeta de vot alhora: **A** (quadrat negre), **B** (quadrat puntejat) o **=** (ratllat, «mantenir la política actual»). La persona operadora compta les mans i introdueix el recompte.
- **Guanya la majoria simple.** Mantenir la política actual compta com una opció més.
- **Empat entre les opcions més votades:** es fa **una segona volta** a l’acte, només entre les opcions empatades i amb les mateixes targetes.
- **Segon empat** → **es manté la política actual**.
- **Cap paquet promogut** → no es vota. **Es manté la política actual**. Sempre hi ha una ronda següent.
- Els punts que no gasta el paquet guanyador es perden.

### 5. Resultats (5 min, o 4 a les rondes 5–6)

1. **Abans que la persona operadora avanci**, qui porta el Registre de les taules guanyadores escriu els dos pronòstics a la cronologia mural.
2. La persona operadora confirma i avança cinc anys, i projecta l’**Informe**, que mostra les prediccions al costat dels canvis reals, amb unitats i valors anteriors.
3. Escriviu el canvi real al costat de cada pronòstic i marqueu-lo amb ✓, ✗ o ?.
4. **La primera resposta és per a algú que va votar en contra** del paquet guanyador.
5. Apunteu a la paret **una frase sobre el que la gent creu que va causar el resultat**. Indiqueu-la com a explicació de les persones participants, tret que l’informe mostri la causa.
6. Esborreu els tapets i torneu-los a la taula central. Les taules es queden els seus fulls.

Si falla el projector, la persona operadora llegeix l’informe en veu alta i omple a mà la cronologia mural.

## Rondes 3–4 i la baralla B opcional

- A la pausa, canvia qui parla, de manera que a les rondes 3–4 intervinguin al ple portaveus que encara no ho han fet.
- Apareixen cartes que s’acaben d’habilitar per les decisions de les rondes 1–2. Una carta amb requisit es pot aprovar a la ronda següent a l’aprovació de la carta que l’habilita. La baralla A no té cartes amb requisit, així que això només passa si es fa servir la baralla B.
- La **baralla B** té vuit cartes més en quatre fulls més. Reparteix-la a la pausa **només si** l’assaig va demostrar que la sala podia amb vuit opcions més (design.md, decisió 5). Si no, deixa-la a la capsa.

## Repetició (2:30–2:45)

| Rellotge | Què cal fer |
|---|---|
| 2:30–2:32 | La sala tria **una decisió discutida**. Per defecte, la ronda amb la votació més ajustada o la que més taules esmentin. |
| 2:32–2:34 | La persona operadora restaura el món d’**abans d’aquella votació** i introdueix el **paquet perdedor** en lloc del guanyador. TODO W5: branca d’una ronda. A l’assaig, fes servir una branca preparada per endavant i digues-ho. |
| 2:34–2:35 | Les taules promotores perdedores recorden les seves dues prediccions. |
| 2:35–2:40 | Avança **una sola ronda**. Compareu els cinc anys següents del paquet real i del substitut, que parteixen del mateix estat. |
| 2:40–2:44 | «Què hauria estat diferent? Qui ho hauria pagat?». |
| 2:44–2:45 | **Torna a la història real** i comprova que a la pantalla torna a haver-hi el final real. |

## Balanç (2:45–3:00)

| Rellotge | Què cal fer |
|---|---|
| 2:45–2:55 | Cada taula té 2,5 minuts. **Heu complert el vostre compromís?** Doneu les dades de l’informe. Digueu **un cost que el tauler no va recollir**. Després responeu, tant com pugueu, la vostra **pregunta que el model no pot respondre**. |
| 2:55–2:58 | Demana que aixequi la mà qui va veure complerta la seva predicció personal, i després qui no. Pregunta a una persona que s’hi va equivocar què se li va escapar. |
| 2:58–3:00 | **Tanca amb un desacord que la sala encara manté.** Escriu-lo a la paret i acaba a temps. |

Preguntes per tenir a mà:

- Quina concessió va fer o rebutjar la vostra taula? La tornaríeu a fer?
- Al final, d’on va sortir la terra?
- Quin resultat va explicar la sala de manera diferent del que mostra l’informe?

## Indicadors per a les prediccions

Les prediccions s’han de poder comprovar a l’Informe, així que ofereix aquesta llista tancada. Els ID dels indicadors i les unitats vénen dels fixtures de W1 (`meta.units`), i els valors inicials de la fila del 2022 sense mesures. Els noms en cada llengua són a `content/strings.csv` (`metric.*`).

| Indicador | ID de l’indicador | Unitat | Valor el 2022 |
|---|---|---|---|
| Emissions | `emissions_gtco2eq` | Gt CO₂-eq l’any | 51,6 |
| Temperatura | `temperature_c` | °C d’escalfament (anomalia) | 1,10 |
| Pressió d’extinció (biodiversitat) | `extinction_rate` | índex adimensional (com menys, millor) | 90,2 |
| Satisfacció | `outlook` | índex adimensional | 30,0 |
| Ús de la terra | `land_use_fraction` | % de la terra habitable inicial, inclosa la protegida | 59 % |

Avís per als primers Resultats: triï el que triï la sala, el model fa un salt el primer any. La temperatura passa d’1,10 a 1,37 °C el 2023 i la satisfacció de 30,0 a 25,9, perquè la temperatura inicial del món no coincideix amb l’historial del model climàtic (model-baseline.md §2). Digues-ho abans dels primers Resultats perquè ningú no ho interpreti com a efecte de la votació.

## Registre d’observació (assaig i pilot)

Apunta a cada ronda:

- qui va esmenar una proposta
- qui va parlar al ple
- quantes propostes van canviar després d’una visita
- el temps d’introducció de la persona operadora
- un pronòstic per coalició i si es va complir

L’assaig se supera quan les dues rondes caben en 20 minuts, almenys tres taules diuen una concessió que van fer o rebutjar i cada taula explica una conseqüència mesurada.

## Qüestions obertes

- **TODO (baralla B):** triar les vuit cartes opcionals i els seus temes, efectes i requisits a partir de model-baseline.md §5.1. `optional-deck-b.html` encara fa servir marcadors de posició.
- Noms dels indicadors i textos de les cartes en castellà i català: `content/strings.csv` (pendent de revisió).
- **TODO W5:** decidir si l’assemblea rebutja el conflicte del carbó o només avisa, segons el punt obert 4 de model-baseline.md. Aquest guió el tracta com a avís.
- **TODO W5/W6:** les operacions de restaurar, branca d’una ronda i tornar a la història real que es fan servir a la demostració inicial i a la repetició.
- **Revisió de l’especificació:** si «La Decisió no es mou mai; els retards surten de Resultats» i la finestra d’un minut per corregir s’han d’escriure a l’especificació assembly-session.
- **Prova amb persones:** l’acceptació real de W2 és una esmena i una votació físiques amb quatre taules i dos tapets. Encara no s’ha fet.
