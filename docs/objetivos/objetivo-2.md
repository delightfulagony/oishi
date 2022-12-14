# Lista de comprobación

Antes de proponer el material de este objetivo para revisión, hazte las siguientes preguntas

## Sobre la estructura del repositorio
- [x] ¿He seguido las mejores prácticas en nombre de las clases y ficheros y disposición de los mismos?

## Sobre el análisis del problema
- [x] ¿Se ha documentado qué análisis se ha hecho sobre el dominio para decir lo que se ha creado?
- [x] ¿Se ha documentado por qué se ha elegido que lo creado sea un objeto valor, una entidad o un agregado?
- [x] ¿He propuesto un producto mínimamente viable, que en general será un solo objeto valor que no dependa de ningún otro (y que sea la base de muchos otros)?.

## Sobre la planificación y la programación
- [x] ¿Los issues representan un problema, y no una tarea?
- [x] ¿Todos los mensajes de commit explican el cambio, y no se limitan a repetir el nombre del fichero que se ha cambiado?
- [x] ¿Los mensajes de commit siguen el [el formato estándar y las buenas prácticas](https://www.theserverside.com/video/Follow-these-git-commit-message-guidelines).
- [x] ¿Se ha hecho una revisión real del código para comprobar que todos los atributos y funciones creadas están respaldadas por una HU?
- [x] ¿Todos los issues creados están asignados a una HU?
- [x] ¿Todos los cambios en el código están asignados a un issue?
- [x] Los issues sobre los que estoy trabajando, ¿han sido asignados al primer milestone?
- [x] ¿Se ha asignado al mismo milestone el PR que se ha hecho?
- [x] ¿Son los milestones sobre los que estoy trabajando productos mínimamente viables? ¿O tengo que solicitar al product manager que cree milestones adicionales o precise de qué producto se trata?

## Retos enfrentados
1. Se ha trabajado siguiendo un flujo "ágil", partiendo del dominio del
problema (las HUs) hasta el dominio de la solución (el código), siguiendo el
flujo HU→ hito→ issue→ mensaje de commit→ código. Este flujo de trabajo permite
también hacer el camino inverso, teniendo cada paso un mayor nivel de detalle,
de forma que dado un fragmento de código se pueda obtener todo el contexto
necesario para comprenderlo. Esto ha supuesto un buen número de problemas para
seguir y respetar este flujo, el problema ha venido principalmente de la
comprensión de este flujo.
2. La colaboración con otras personas añade "delays" que inevitablemente
ralentizan el desarrollo.
3. Trabajar en un repo en el que no se tienen todos los permisos necesarios
también añade ciertos "delays" cuando hay que solucionar al product owner
que haga ciertos cambios para los que no se tienen permisos.
