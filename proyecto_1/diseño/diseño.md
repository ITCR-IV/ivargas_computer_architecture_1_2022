---
header-includes: yes
---
<!-- geometry: "left=1.6cm,right=1.6cm,top=1.6cm,bottom=1.6cm" -->

# Documentación de diseño

## Requerimientos

- Programa en ASM que realice interpolación bilineal sobre un conjunto de pixeles de una imagen blanco y negro.
- Programa en lenguaje de alto nivel para interactuar con el usuario y mostrar la imagen y el resultado.
- Integración entre lenguaje de alto nivel y programa ASM que posiblemente corra en emulador.

## Opciones de solución

### ASM

1. RISC-V: Es un lenguaje que ya manejo bastante bien y con facilidad. Tendría que correrlo en un emulador.
2. x86: Es un lenguaje que no manejo y nunca he utilizado. Puede correr nativamente en mi computador personal.

### Lenguajes alto nivel

1. C: Es muy simple. Se utiliza muy comúnmente para interactuar con código ASM nativo. No tiene un sistema de manejo de dependencias nativo o moderno y ocuparía alguna librería como SDL para mostrar los resultados de imágenes.
2. Rust: Es bastante complejo. No es muy común interactuar con código ASM nativo pero sí tiene las facilidades para hacerlo. Tiene un sistema de manejo de dependencias moderno y muy elegante. De igual forma ocuparía alguna librería para mostrar las imágenes. Una ventaja de este lenguaje que solo aplica por este proyecto ser en un contexto educativo es que me gustaría mucho profundizar mi conocimiento al respecto. También existen muy buenas librerías para interacción de CLI si deseara crear el programa de esa manera.
3. Python: Es sumamente sencillo. Es un buen lenguaje de scripting por lo que bastante fácilmente podría interactuar con herramientas externas para correr ASM en un emulador. Tiene un sistema de manejo de dependencias relativamente moderno y fácil de usar. De igual forma ocuparía alguna librería para mostrar las imágenes. Sumamente fácil crear interfaces de scripting o visuales. Tiene desventaja en el ámbito educativo porque no estoy particularmente interesado en ampliar mi conocimiento de Python y no aprendería realmente nada muy útil del lenguaje realizando este proyecto en él.

### Sistema de integración

1. Scripting: En vez de llamar ASM directamente dentro del código como en C o Rust, podría llamar un emulador que ejecute el código por fuera, lo cual requeriría un shell script o parecido para juntar el programa de alto nivel y el programa ASM. Tiene una ventaja de que es fácil usar cualquiera de las opciones de ensamblador.
2. Código embebido: Si el código ASM está embebido dentro del ejecutable esto requeriría que utilice ASM x86 para correr el código ASM nativamente y utilizar Rust o C que tienen esta capacidad de embeber ASM en su ejecutable. Este método produciría un programa muy eficiente (aunque este no es un objetivo del proyecto) y un solo ejecutable binario que se encargue de todo, sin tener que "pegar" diferentes partes con un script y utilizar un emulador externo. También esta opción tiene una ventaja de este ser un contexto educativo porque sí me interesa aprender cómo sería embeber código ASM en una aplicación real.


### Comparación de opciones

## Propuesta final
