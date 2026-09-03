# Half-Earth Socialism: The Game — Quarter Earth

## ¿Qué es Quarter Earth?

Quarter Earth es un fork de [Half-Earth Socialism](https://github.com/frnsys/half_earth) adaptado para convertirse en un taller de aproximadamente una hora, dirigido por una persona moderadora, para la asamblea acTe. Conserva la premisa de planificación planetaria del juego original: el grupo elige políticas que afectan al clima, la biodiversidad, la energía, la alimentación y el uso del suelo.

Este repositorio contiene el modo de taller para navegador: una experiencia autocontenida y pensada para proyectarse o compartirse durante una sesión facilitada. Para consultar toda la documentación del juego original, visita el [README del repositorio principal](https://github.com/frnsys/half_earth).

## Ejecutarlo en el navegador

En una copia recién descargada, instala una sola vez los requisitos para compilar la versión web:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
cd game/assets/js && npm ci
```

Después, desde la raíz del repositorio, inicia el servidor de desarrollo:

```bash
just run-web
```

Abre [http://localhost:8080/](http://localhost:8080/) en un navegador. El servidor detecta los cambios en el código y vuelve a compilar el juego; déjalo ejecutándose mientras desarrollas y detenlo con <kbd>Ctrl</kbd>+<kbd>C</kbd>.

Para ejecutar la versión simplificada para talleres, abre [http://localhost:8080/?workshop=1](http://localhost:8080/?workshop=1). El mundo de taller está incluido en la compilación web, por lo que no es necesario cargar ningún archivo de mundo por separado.

Para jugar en español, abre cualquiera de las dos direcciones y selecciona `es` en el menú de idioma situado en la esquina superior derecha de la pantalla de inicio.

## What is Quarter Earth?

Quarter Earth is a fork of [Half-Earth Socialism](https://github.com/frnsys/half_earth) adapted into an approximately one-hour, moderator-led workshop for the acTe assembly. It keeps the original game's planetary-planning premise: the group chooses policies that affect climate, biodiversity, energy, food, and land use.

This repository contains the browser workshop mode: a self-contained experience designed for projection or shared play during a facilitated session. For the original game's complete documentation, see the [main repository README](https://github.com/frnsys/half_earth).

## Run in a browser

From a fresh checkout, install the web build prerequisites once:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
cd game/assets/js && npm ci
```

Then, from the repository root, start the development server:

```bash
just run-web
```

Open [http://localhost:8080/](http://localhost:8080/) in a browser. The server watches the source files and rebuilds the game after changes; leave it running while developing and stop it with <kbd>Ctrl</kbd>+<kbd>C</kbd>.

To run the simplified, facilitator-led version, open [http://localhost:8080/?workshop=1](http://localhost:8080/?workshop=1). Workshop mode has its world embedded in the web build, so no separate world file needs to be loaded.

To play in Spanish, open either URL and select `es` from the language menu in the top-right corner of the start screen.
