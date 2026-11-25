# Guion de la asamblea: guía para la persona facilitadora

> Traducción de run-sheet.md, pendiente de revisión por una persona hablante nativa (ver content/README.md). Si hay diferencias, manda la versión inglesa.

Este guion cubre una asamblea de Quarter Earth de tres horas para 12–24 personas repartidas en cuatro mesas. Hace falta una persona facilitadora, un portátil para operar el modelo, un proyector y una pared. Tenlo abierto en el portátil durante la sesión.

El material en papel está en [`print/README.md`](../print/README.md). Las reglas siguen `openspec/changes/assembly-workshop/specs/assembly-session/spec.md`, en la carpeta personal del proyecto. Si una regla de aquí contradice esa especificación, manda la especificación: primero se modifica la especificación y después este guion.

> Los datos de las cartas, los requisitos, el conflicto oculto del carbón, los tiempos, los valores iniciales y los años de cada ronda vienen de `references/assembly/model-baseline.md` de W1 (commit 45277e6e) y de la fila de 2022 de `fixtures/a_no_action/ozone_appended.json`. Los TODO pendientes están en Asuntos abiertos.

## El reloj: 180 minutos

| Inicio | Fin | Min | Bloque | Bloque de la especificación |
|---|---|---|---|---|
| 0:00 | 0:15 | 15 | Sentar a las mesas, leer el planeta inicial, mostrar una carta en un mundo de prueba y reiniciar | Apertura y práctica **30** |
| 0:15 | 0:30 | 15 | Compromisos y preguntas, acuerdo de práctica entre dos mesas, predicciones personales | ″ |
| 0:30 | 0:50 | 20 | Ronda 1 (baraja A) | Rondas 1–2 **40** |
| 0:50 | 1:10 | 20 | Ronda 2 | ″ |
| 1:10 | 1:20 | 10 | Pausa. Dejar a la vista la cronología y las propuestas | Pausa **10** |
| 1:20 | 1:40 | 20 | Ronda 3. Cambia quién hace de portavoz, cartas recién disponibles | Rondas 3–4 **40** |
| 1:40 | 2:00 | 20 | Ronda 4 | ″ |
| 2:00 | 2:15 | 15 | Ronda 5. Las mesas revisan propuestas anteriores o renuevan prioridades | Rondas 5–6 **30** |
| 2:15 | 2:30 | 15 | Ronda 6. La última votación tiene que hacerse a tiempo | ″ |
| 2:30 | 2:45 | 15 | Repetición de una decisión discutida | Repetición **15** |
| 2:45 | 3:00 | 15 | Balance | Balance **15** |

Comprobación: 30 + 40 + 10 + 40 + 30 + 15 + 15 = **180**.

Años de cada ronda (model-baseline.md §3). Las mesas planifican en el primer año de cada ronda y la partida termina en 2052.

| Ronda | Año de planificación | Años que cubre |
|---|---|---|
| 1 | 2022 | 2022 → 2027 |
| 2 | 2027 | 2027 → 2032 |
| 3 | 2032 | 2032 → 2037 |
| 4 | 2037 | 2037 → 2042 |
| 5 | 2042 | 2042 → 2047 |
| 6 | 2047 | 2047 → 2052 |

**Las cartas tardan unos cuatro años en entrar en funcionamiento.** Una carta aprobada en el año de planificación Y está en construcción durante los tres años siguientes. Se activa en el año Y+4 y cambia la producción por primera vez en el último año de la ronda, Y+4 → Y+5. Por eso, la fase de Resultados justo después de una votación muestra **como mucho un año de efecto**, y el efecto completo aparece en las rondas siguientes. Dilo a la sala a las 0:15 y otra vez antes de los primeros Resultados, y tenlo en cuenta al valorar las predicciones.

### Dentro de una ronda

| Fase | Ronda de 20 min (1–4) | Ronda de 15 min (5–6) |
|---|---|---|
| Lectura del año | 0–2 (2) | 0–2 (2) |
| Preparar una propuesta | 2–6 (4) | 2–5 (3) |
| Negociación | 6–12 (6) | 5–9 (4) |
| Decisión | 12–15 (3) | 9–11 (2) |
| Resultados | 15–20 (5) | 11–15 (4) |
| **Total** | **20** | **15** |

Los retrasos se descuentan de **Resultados**. La Decisión nunca se mueve. Una segunda vuelta o un paquete no válido consumen alrededor de un minuto de Resultados; cuando pase, sáltate la pregunta sobre sorpresas en la siguiente Lectura en vez de ir con retraso.

## Antes de que llegue la gente

- [ ] El material de diez hojas está impreso y recortado (ver `print/README.md`). En cada mesa hay una baraja A, un mandato y un bolígrafo.
- [ ] Los dos tapetes de coalición están en fundas A4 sobre la **mesa central**, junto a la persona operadora, con dos rotuladores borrables y un trapo.
- [ ] Cada mesa tiene siete hojas de paquete: una para la práctica y una por ronda. Cada persona tiene un juego de tarjetas de voto (A / B / =) y una hoja de predicción.
- [ ] Cronología mural: las hojas de las rondas 1–6 y la de repetición están pegadas en orden, con espacio al lado para el compromiso y la pregunta de cada mesa.
- [ ] El portátil tiene cargado el mundo fijado. Se ha probado restaurar y repetir en este mismo portátil.
  - Para el ensayo (W4), usa el modo taller existente con `WORKSHOP=1 just run` o `?workshop=1`. Carga el mundo desde el menú principal. No uses `DEBUG_VIEW=Plan`. Las notas de arranque de M1 están en `references/simplification/m1-run-sheet.md`.
  - TODO W5/W6: el punto de entrada de la asamblea y sus vistas Planeta / Votación / Informe sustituyen este paso.
- [ ] La lista de cartas disponibles está en la hoja de la persona operadora. En la ronda 1 están disponibles las ocho cartas de la baraja A. Las cartas con requisito y el conflicto del carbón están en la comprobación de la persona operadora, más abajo.

## Mesas y mandatos

| Código | Mesa | Responde de | Pregunta que puede llevar |
|---|---|---|---|
| AT | Alimentación y tierra | Alimentar a la población durante la transición | ¿De dónde saldrá la tierra? |
| EI | Energía e industria | Mantener la producción mientras se recortan emisiones | ¿Qué sustituye al suministro que retiramos? |
| NV | Nivel de vida | Defender el bienestar diario y el acceso a lo básico | ¿Quién cambia primero su consumo? |
| MV | Mundo vivo | Dejar hábitats y biodiversidad en condiciones de recuperarse | ¿Qué protección sobrevive al próximo presupuesto? |

Todas las mesas tienen las mismas cartas y una responsabilidad distinta. Nadie hace de lobby obstruccionista ni habla en nombre de una comunidad afectada.

## Roles rotativos

Cada mesa cubre estos roles en cada ronda y apunta los nombres en la cuadrícula de roles de su hoja de mandato.

| Rol | Qué hace |
|---|---|
| **Enlace** | Lleva la hoja y las cartas de la mesa a otras mesas durante la Negociación y trae de vuelta las ofertas |
| **Registro** | Rellena la hoja de paquete, los costes y las dos predicciones. Firma un tapete en nombre de la mesa cuando la mesa está de acuerdo |
| **Control del tiempo** | Vigila el reloj proyectado. Lleva el tapete a la mesa central **en cuanto está firmado**, y nunca después del final de la Negociación. Cuenta las manos de la mesa si se le pide |
| **Portavoz** | Lee el año en voz alta en la Lectura. Las dos mesas promotoras de un paquete eligen a **una** de sus dos personas portavoces para defenderlo en 30 segundos en la Decisión |

La mesa decide quién hace cada rol, con estos límites:

- **Se rota cada ronda.** Cada rol pasa a la persona de la izquierda.
- **El enlace rota por completo.** Nadie repite como enlace hasta que todas las personas de la mesa lo hayan sido una vez.
- **El enlace vuelve antes de firmar.** El enlace tiene que estar de vuelta en su mesa antes de que esta firme cualquier tapete, y el tapete tiene una casilla para marcarlo. Un enlace no puede comprometer a su mesa desde otra mesa.
- **Las mesas pequeñas acumulan roles.** En una mesa de tres, Control del tiempo y Portavoz van juntos. En una mesa de cinco o seis, las personas sobrantes no tienen rol fijo: reciben visitas y comprueban costes.
- **Las aportaciones por escrito cuentan.** Cualquiera puede escribir una enmienda o una pregunta en el dorso de la hoja en lugar de decirla en voz alta, y quien lleva el Registro la lee. Ofrécelo explícitamente en la ronda 1 para que las voces más rápidas no dirijan todas las mesas.

## Apertura y práctica (0:00–0:30)

| Reloj | Qué hacer |
|---|---|
| 0:00–0:05 | Sienta a la gente en cuatro mesas de tres a seis personas. Reparte los mandatos. Se eligen los roles de la práctica. |
| 0:05–0:10 | Proyecta el planeta inicial y leedlo juntos: año, emisiones, temperatura, presión de extinción, satisfacción y el indicador de tierra. Cada mesa lee en voz alta su recuadro de datos. Son los valores de 2022 de la simulación sin medidas de W1. |
| 0:10–0:15 | **Muestra una carta** en una copia de prueba del mundo. Enseña una carta sobre un tapete, su coste frente al presupuesto de 30 puntos, la introducción en el portátil, el avance y el informe. Después **reinicia** al mundo inicial delante de la sala, para que todo el mundo vea que no se ha conservado nada. |
| 0:15–0:20 | Cada mesa escribe su **compromiso público**: un indicador, una dirección, un objetivo y un año. También escribe **una pregunta que el modelo no puede responder**. Quien lleva el Registro copia ambos en la pared. |
| 0:20–0:27 | **Acuerdo de práctica.** AT trabaja con MV, y EI con NV. Cada pareja arma un paquete sobre un tapete, rellena el presupuesto y las comprobaciones, escribe dos predicciones y firma. No se vota ni se introduce nada. Después se borran los tapetes. |
| 0:27–0:30 | Cada persona escribe una **hoja de predicción personal** y la guarda hasta el balance. |

Di esto a las 0:15: «Cada paquete tiene 30 puntos, que no se acumulan de una ronda a otra. Un paquete necesita la firma de dos mesas. Cada mesa puede firmar un paquete por ronda, así que como mucho llegan dos paquetes a votación. Firmar no obliga a nadie a votar de una manera. Una carta tarda unos cuatro años en entrar en funcionamiento, así que casi todo lo que aprobéis se notará en rondas posteriores».

## Desarrollo de cada ronda (rondas 1–6)

### 1. Lectura del año (2 min)

- La persona operadora proyecta la vista **Planeta**. La persona portavoz de cada mesa lee el titular en voz alta.
- Pregunta a una mesa: «¿Qué os sorprendió en la ronda anterior?». No lo hagas en la ronda 1 ni después de una ronda que se haya alargado.
- La persona operadora anuncia las **cartas disponibles**: las que ya tienen sus requisitos en vigor, menos las ya aprobadas. Las mesas ponen bocabajo las que no están disponibles.
  - **Una carta solo se puede aprobar una vez.** El motor solo acepta una carta mientras está `Inactive`. Una vez aprobada pasa a `Building` y después a `Active`, así que no se puede volver a aprobar (model-baseline.md §2, §5.1). Es una regla del motor, no de la casa.
  - Ninguna carta de la baraja A tiene requisitos.
  - Una carta con requisito se puede aprobar en el **año de planificación siguiente a la aprobación de la carta que la habilita**, es decir, en la ronda siguiente. Requisitos en la baraja completa: Agricultura regenerativa → Carne cultivada. Impulso solar, Impulso eólico o Expansión nuclear → Electrificación masiva. Restaurar y proteger los ecosistemas → Gestión de la radiación solar (GRS).

### 2. Preparar una propuesta (4 min, o 3 en las rondas 5–6)

- Primero, **30 segundos de silencio**. Cada persona elige en privado una prioridad tocando una carta.
- Quien lleva el Registro rellena una **hoja de paquete** con 1–3 cartas disponibles **distintas** que cuesten **30 puntos o menos**, más **dos predicciones de dirección**: un indicador y ↑ o ↓.
- Las medidas que ya están en vigor siguen funcionando con sus propias reglas. El presupuesto solo compra decisiones nuevas. Los puntos que no se gastan **se pierden** al final de la ronda.
- En las rondas 5–6, las mesas pueden volver a presentar o revisar una hoja anterior en lugar de empezar de cero.

### 3. Negociación (6 min, o 4 en las rondas 5–6)

- El enlace lleva la hoja, con sus cartas, a otra mesa. El resto recibe visitas, comprueba costes o prepara enmiendas. Las mesas pueden plantear su pregunta del mandato a los enlaces que las visitan.
- **Para llegar a votación**, dos mesas se ponen de acuerdo en un paquete. Su Control del tiempo recoge un tapete libre en la mesa central. Las cartas van en las casillas y los costes debajo. Se suma el total, se sombrea **una casilla del indicador por cada bloque de 5 puntos** y se marcan las seis comprobaciones.
- **Enmienda.** Una carta sustituida en el tapete lleva marcada la casilla «Sustituida», junto con qué carta **sustituye** y qué mesa la **propuso**. Lo que identifica la enmienda es la marca escrita. Nunca se usa el color para esto.
- **Firma.** Cada enlace vuelve a su mesa y marca la casilla. Quien lleva el Registro firma por la mesa, y las dos predicciones se escriben en el tapete.
  - **Cualquier cambio después de una firma anula las dos firmas**, y las dos mesas vuelven a firmar.
- **Límites.** Cada mesa firma **un tapete por ronda**. Hay dos tapetes, así que como mucho llegan dos paquetes a votación. Si firman las cuatro mesas, cada una promueve exactamente un paquete.
- **Entrega.** El Control del tiempo lleva el tapete a la mesa central **en cuanto las dos mesas han firmado**, para que la persona operadora lo compruebe y lo introduzca mientras sigue la Negociación. El final de la Negociación es el último momento para entregarlo, no el comienzo de la introducción. Un tapete con una sola firma al acabar el plazo no llega a votación.

### Comprobación de la persona operadora (durante la Negociación, a medida que llegan los tapetes)

Introduce cada tapete en la vista de facilitación en cuanto llegue. Hasta que W6 construya esa vista, por ejemplo en el ensayo W4, comprueba a mano las seis casillas del tapete y aprueba las cartas en la interfaz del taller existente después de la votación. Si se rechaza, lee el motivo en voz alta. Las mesas promotoras lo corrigen **en la mesa central**, usando como máximo un minuto de Resultados.

| No es válido porque | Qué ve o comprueba la persona operadora | La mesa lo corrige |
|---|---|---|
| Supera el presupuesto (total > 30) | La suma de los costes de las casillas, que coincide con los bloques del indicador | Quitando o cambiando una carta hasta que el total sea 30 o menos |
| Carta repetida | El mismo ID de carta en dos casillas | Quitando la repetida y, si quiere, añadiendo otra carta distinta |
| Carta ya aprobada | La carta se aprobó en una ronda anterior. El motor solo acepta cartas que siguen `Inactive` | Cambiándola por una carta disponible |
| Requisito todavía no aprobado | La línea «Requiere» de la carta frente a las cartas aprobadas en rondas **anteriores**. Una carta habilitadora aprobada en esta ronda todavía no cuenta. Ninguna carta de la baraja A tiene requisitos (ver los requisitos en Lectura del año) | Cambiando la carta o quitándola |
| Exclusión declarada | No hay ninguna en la baraja actual | — |
| **Aviso, no rechazo: conflicto del carbón** | **Eliminar el carbón** junto con **Impulso solar** (o Expansión nuclear), en el mismo paquete o en rondas distintas. El carbón empieza con el 35 % de la electricidad, que no basta para los dos cambios. El motor acepta las dos, cobra las dos y las muestra como activas, pero la que se aplica en segundo lugar no hace nada (model-baseline.md §5.1). La línea «No se combina con» de las dos cartas lo indica | Lee el aviso en voz alta. La mesa se queda con una de las dos o paga, sabiéndolo, por una carta que no hará nada |
| Mesa promotora repetida (una mesa en los dos tapetes) | Los códigos de mesa de los dos tapetes | La mesa elige un tapete. El otro necesita la firma de otra segunda mesa o no llega a votación |
| Menos de dos mesas distintas | Los códigos de mesa | Buscando una segunda mesa; si no, el paquete no llega a votación |
| Más de tres cartas o ninguna | El número de casillas ocupadas | Dejándolo en 1–3 |

Si al empezar la Decisión el paquete sigue sin ser válido, **no va a votación** en esa ronda. Cualquier cambio obliga a las dos mesas a volver a firmar, igual que durante la Negociación.

### 4. Decisión (3 min, o 2 en las rondas 5–6)

- La vista **Votación** muestra cada paquete con sus cartas, costes, mesas promotoras y predicciones. Las predicciones quedan fijadas **antes** de votar y la persona operadora ya las ha introducido.
- Cada **paquete** tiene **30 segundos**, con una sola voz por paquete. Eso suma 1 minuto de defensas y deja 1–2 minutos para votar y contar en los dos tipos de ronda.
- **Votación.** Cada participante tiene un voto y vota a título individual. Cuenta atrás: «3, 2, 1, ¡arriba!». Todo el mundo levanta **una** tarjeta de voto a la vez: **A** (cuadrado negro), **B** (cuadrado punteado) o **=** (rayado, «mantener la política actual»). La persona operadora cuenta las manos e introduce el recuento.
- **Gana la mayoría simple.** Mantener la política actual cuenta como una opción más.
- **Empate entre las opciones más votadas:** se hace **una segunda vuelta** en el acto, solo entre las opciones empatadas y con las mismas tarjetas.
- **Segundo empate** → **se mantiene la política actual**.
- **Ningún paquete promovido** → no se vota. **Se mantiene la política actual**. Siempre hay una ronda siguiente.
- Los puntos que no gasta el paquete ganador se pierden.

### 5. Resultados (5 min, o 4 en las rondas 5–6)

1. **Antes de que la persona operadora avance**, quien lleva el Registro de las mesas ganadoras escribe los dos pronósticos en la cronología mural.
2. La persona operadora confirma y avanza cinco años, y proyecta el **Informe**, que muestra las predicciones junto a los cambios reales, con unidades y valores anteriores.
3. Escribid el cambio real junto a cada pronóstico y marcadlo con ✓, ✗ o ?.
4. **La primera respuesta es para alguien que votó en contra** del paquete ganador.
5. Apuntad en la pared **una frase sobre lo que la gente cree que causó el resultado**. Señaladla como explicación de las personas participantes, salvo que el informe muestre la causa.
6. Borrad los tapetes y devolvedlos a la mesa central. Las mesas se quedan con sus hojas.

Si falla el proyector, la persona operadora lee el informe en voz alta y rellena a mano la cronología mural.

## Rondas 3–4 y la baraja B opcional

- En la pausa, cambia quién habla, de modo que en las rondas 3–4 intervengan en el pleno portavoces que todavía no lo han hecho.
- Aparecen cartas recién disponibles por las decisiones de las rondas 1–2. Una carta con requisito se puede aprobar en la ronda siguiente a la aprobación de la carta que la habilita. La baraja A no tiene cartas con requisito, así que esto solo pasa si se usa la baraja B.
- La **baraja B** tiene ocho cartas más en cuatro hojas más. Repártela en la pausa **solo si** el ensayo demostró que la sala podía con ocho opciones más (design.md, decisión 5). Si no, déjala en la caja.

## Repetición (2:30–2:45)

| Reloj | Qué hacer |
|---|---|
| 2:30–2:32 | La sala elige **una decisión discutida**. Por defecto, la ronda con la votación más ajustada o la que más mesas mencionen. |
| 2:32–2:34 | La persona operadora restaura el mundo de **antes de esa votación** e introduce el **paquete perdedor** en lugar del ganador. TODO W5: rama de una ronda. En el ensayo, usa una rama preparada de antemano y dilo. |
| 2:34–2:35 | Las mesas promotoras perdedoras recuerdan sus dos predicciones. |
| 2:35–2:40 | Avanza **una sola ronda**. Comparad los cinco años siguientes del paquete real y del sustituto, que parten del mismo estado. |
| 2:40–2:44 | «¿Qué habría sido distinto? ¿Quién lo habría pagado?». |
| 2:44–2:45 | **Vuelve a la historia real** y comprueba que en pantalla vuelve a estar el final real. |

## Balance (2:45–3:00)

| Reloj | Qué hacer |
|---|---|
| 2:45–2:55 | Cada mesa tiene 2,5 minutos. **¿Habéis cumplido vuestro compromiso?** Dad los datos del informe. Nombrad **un coste que el panel no recogió**. Después responded, en lo posible, a vuestra **pregunta que el modelo no puede responder**. |
| 2:55–2:58 | Pide que levante la mano quien vio cumplida su predicción personal, y después quien no. Pregunta a una persona que se equivocó qué se le escapó. |
| 2:58–3:00 | **Cierra con un desacuerdo que la sala todavía mantiene.** Escríbelo en la pared y termina a tiempo. |

Preguntas para tener a mano:

- ¿Qué concesión hizo o rechazó vuestra mesa? ¿La volveríais a hacer?
- Al final, ¿de dónde salió la tierra?
- ¿Qué resultado explicó la sala de forma distinta a lo que muestra el informe?

## Indicadores para las predicciones

Las predicciones tienen que poder comprobarse en el Informe, así que ofrece esta lista cerrada. Los ID de los indicadores y las unidades vienen de los fixtures de W1 (`meta.units`), y los valores iniciales de la fila de 2022 sin medidas. TODO W3: los nombres en cada idioma.

| Indicador | ID del indicador | Unidad | Valor en 2022 |
|---|---|---|---|
| Emisiones | `emissions_gtco2eq` | Gt CO₂-eq al año | 51,6 |
| Temperatura | `temperature_c` | °C de calentamiento (anomalía) | 1,10 |
| Presión de extinción (biodiversidad) | `extinction_rate` | índice adimensional (cuanto menos, mejor) | 90,2 |
| Satisfacción | `outlook` | índice adimensional | 30,0 |
| Uso de la tierra | `land_use_fraction` | % de la tierra habitable inicial, incluida la protegida | 59 % |

Aviso para los primeros Resultados: elija lo que elija la sala, el modelo da un salto en su primer año. La temperatura pasa de 1,10 a 1,37 °C en 2023 y la satisfacción de 30,0 a 25,9, porque la temperatura inicial del mundo no coincide con el historial del modelo climático (model-baseline.md §2). Dilo antes de los primeros Resultados para que nadie lo interprete como efecto de la votación.

## Registro de observación (ensayo y piloto)

Apunta en cada ronda:

- quién enmendó una propuesta
- quién habló en el pleno
- cuántas propuestas cambiaron después de una visita
- el tiempo de introducción de la persona operadora
- un pronóstico por coalición y si se cumplió

El ensayo se supera cuando las dos rondas caben en 20 minutos, al menos tres mesas nombran una concesión que hicieron o rechazaron y cada mesa explica una consecuencia medida.

## Asuntos abiertos

- **TODO (baraja B):** elegir las ocho cartas opcionales y sus temas, efectos y requisitos a partir de model-baseline.md §5.1. `optional-deck-b.html` todavía usa marcadores de posición.
- **TODO W3:** los nombres de los indicadores y los textos de las cartas en castellano y catalán.
- **TODO W5:** decidir si la asamblea rechaza el conflicto del carbón o solo avisa, según el punto abierto 4 de model-baseline.md. Este guion lo trata como aviso.
- **TODO W5/W6:** las operaciones de restaurar, rama de una ronda y volver a la historia real que se usan en la demostración inicial y en la repetición.
- **Revisión de la especificación:** si «La Decisión nunca se mueve; los retrasos salen de Resultados» y la ventana de un minuto para corregir deben escribirse en la especificación assembly-session.
- **Prueba con personas:** la aceptación real de W2 es una enmienda y una votación físicas con cuatro mesas y dos tapetes. Todavía no se ha hecho.
