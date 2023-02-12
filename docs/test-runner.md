# Test runner

## Requisitos

Se busca un test runner que:
- Soporte el formato
[TAP (Test Anything Protocol)](https://en.wikipedia.org/wiki/Test_Anything_Protocol)
- Esté actualizado
- Como consecuencia del requisito anterior, esté mantenido.

Opcionalmente se valora que:
- No requiera permisos de escritura en el repositorio donde se definen los
tests para su ejecución.

## Elección

Para el test runner se ha escogido: 
[`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
Este es el test runner estándar de rust, soporta el formato TAP y es parte de
la biblioteca estándar de Rust, por lo que se mantiene regularmente. Además
permite especificar el directorio en el que genera los ficheros necesarios
por medio de la variable de entorno `CARGO_TARGET_DIR`.

Se escogió inicialmente nextest, pero se ha descartado por no respetar las
directivas de $CARGO_TARGET_DIR.

## Otras opciones barajadas

Se han valorado también las siguientes opciones:

- [`nextest`](https://github.com/nextest-rs/nextest)

Éste es un test runner que viene a mejorar el rendimiento y la legibilidad de
`cargo test`, por medio de un modelo de ejecución que incorpora el paralelismo
de los tests desde su diseño.

Está mantenido y actualizado, soporta TAP y permite definir el directorio
de trabajo por medio de la variable de entorno `CARGO_TARGET_DIR`.
