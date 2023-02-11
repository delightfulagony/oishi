# Requisitos para la elección de una imagen para CI
## Prioritario
- Peso de la imagen:
La imagen se descarga cada vez que se ejecutan los tests, por lo que es
importante que no sea muy pesada, de lo contrario los tiempos de test serán
muy largos lo que afecta negativamente al desarrollo.
- Velocidad (Muy ligada al peso):
Rust es un lenguaje notoriamente lento en compilación, por lo que al igual que
en el punto anterior interesa que la imagen sea lo más rápida posible para
facilitar el desarrollo.
- Disponibilidad de las herramientas necesarias:
Esto casi huelga decirlo, la imagen debe contener todas las herramientas
necesarias para contruir y ejecutar los tests de la aplicación.
- `stable` vs `edge`:
Se priorizará una imagen lo más actualizada posible, ya que no pasa nada si
falla interesará que puedan darse todos los problemas lo antes posible `fail
fast, fail early`. Al no estar en producción la estabilidad no es una
prioridad, más bien al contrario.

## No prioritario
- Minimizar la superficie de ataque:
Dado que no es una imagen que vaya a usarse en producción, el objetivo es que
disponga de todo lo necesario para ejecutar los tests, no que su software sea
mínimo.
- Estable en el tiempo:
Esta es una imagen que se lanzará, ejecutará los tests y luego se eliminará,
por tanto no es necesario que pueda ejecutarse continuamente durante un periodo
de tiempo largo sin fallos.

# Imagen elegida

Se elige `rust:alpine` como imagen base por ser una imagen basada en alpine,
ligera y actualizada, cumple con todos los requisitos establecidos excepto con
la disponibilidad de todas las herramientas necesarias para ejecución de los
tests, cosa que será solventada en la construcción del dockerfile, instalando
nextest y justfile.

Ninguna de las herramientas barajadas incluía soporte para justfile y nextest
de todas formas.

También es una imagen oficial mantenida por la organización detrás de Rust,
dando por tanto ciertas garantías de mantenimiento.

# Opciones barajadas

- `rust:latest`
	- Pros:
		- Imagen oficial por lo que se encuentra mantenida
		- Contiene todo el entorno de desarrollo que podría necesitarse
		- Actualizada (edge)
	- Cons:
		- 500MB Pesada, al contener todo aquello que podría necesitar un
		proyecto de Rust contiene muchas cosas que no necesitaríamos para
		simplemente la ejecución de tests.
- `rust:alpine`
	- Pros:
		- Imagen oficial por lo que se encuentra mantenida
		- Actualizada (edge)
		- 261MB Ligera, se trata de una image mínima, que contiene lo justo y
		necesario para ejecutar Rust.
	- Cons:
		- Está basada en musl-libc en lugar de glibc, lo que puede crear
		problemas de compatibilidad, concretamente nextest no garantiza soporte
		para musl, aunque de facto sí lo tiene.
		- Requiere instalar las distintas herramientas que se necesiten en la
		imagen, en algunos casos manualmente ya que no se encuentran en los
		repositorios de alpine (justfile y nextest).
- `cimg/rust`
	- Pros:
		- Imagen oficial (aunque de otra organización) por lo que se encuentra
		mantenida
		- Contiene todo el entorno de desarrollo que podría necesitarse
	- Cons:
		- 632MB Pesada
		- No cuenta con versión `latest` por lo que no existe la opción de
		mantenerla actualizada automáticamente.
- `okteto/rust`
	- Pros:
		- Contiene todo el entorno de desarrollo que podría necesitarse.
	- Cons:
		- 652MB Pesada
		- Sin actualizaciones desde hace 2 años (sin mantener)

- `totapia/nextest`
	- Pros:
		- Es una imagen específica para nextest, lo que facilita ejecutar
		los tests sin necesidad de instalar nada extra
	- Cons:
		- Sin actualizaciones desde hace 3 años (sin mantener)
