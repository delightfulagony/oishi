# Biblioteca de aserciones

## TL:DR (Too Long, Don't Read)

Se escoge la biblioteca de aserciones
[`speculoos`](https://github.com/oknozor/speculoos)

## Requisitos

Se busca una biblioteca de aserciones que sea:
- Sencilla de usar
- "Verbose", es decir explícita y fácil de entender
- Actualizada, Rust es un lenguaje relativamente joven y sujeto a cambios de
vez en cuando, la biblioteca de aserciones ha de ser moderna y adaptarse a
esos cambios.
- Como consecuencia clara de el requisito anterior, debe de estar mantenida.

## Elección

Para la biblioteca de aserciones se ha escogido
[`speculoos`](https://github.com/oknozor/speculoos)

Inspirada por los frameworks de aserciones "fluidos" como Google Truth,
es un fork mantenido de [`espectral`](https://github.com/cfrancia/spectral).
Además, la sintaxis es clara y sencilla, por lo que es fácil de usar y de leer.

## Otras opciones barajadas

Se han valorado también las siguientes opciones:

- La biblioteca de aserciones estándar de Rust
[`std::assert`](https://doc.rust-lang.org/std/macro.assert.html).
Incluye pocas funcionalidades, son aserciones "típicas", poco legibles.

- [`more-asserts`](https://docs.rs/more-asserts/latest/more_asserts/)
complementa la librería estándar de aserciones con algunas funcionalidades más,
sigue teniendo la misma sintaxis y la funcionalidad sigue siendo limitada.

- [`spectral`](https://github.com/cfrancia/spectral) es más legible e incluye
más funcionalidad que las anteriores, pero no está mantenida desde hace 6
años, por lo que se descarta.

- [`galvanic-assert`](https://github.com/mindsbackyard/galvanic-assert) esta
es probablemente la más legible y fácilmente entendible de todas, sin embargo
al igual que `spectral` lleva sin mantenerse desde hace 5 años, por lo que
también se descarta.
