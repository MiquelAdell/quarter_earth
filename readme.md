# Half-Earth Socialism: The Game

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

For the full README for the original game, see the [main repository](https://github.com/frnsys/half_earth).
