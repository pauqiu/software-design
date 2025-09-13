# Tarea 2 - Refactorización de código

**Diseño de Software</p>**
**Estudiante:** Paulette Zamora Hernández
## Problemas identificados
- Nombres de variables poco significativos.
    ~~~ 
    l = Libro(titulo, autor, genero, paginas, anio)
    ~~~
- Violación al principio de Single Reponsability. La clase Libro mezcla lógica interna con lógica de impresión de datos. Además, la clase librería maneja los datos de los libros y también genera el reporte.
- Código repetitivo. 
    ~~~
    if self.genero == 'novela':
        base = 50
        extra = self.paginas / 10
    elif self.genero == 'ciencia':
        base = 70
        extra = self.paginas / 5
    elif self.genero == 'historia':
        base = 40
        extra = self.paginas / 8
    else:
        base = 10
        extra = 0
    return base + extra
    ~~~
- Efectos secundarios. En el método *generar_reporte* además de imprimir los datos se cuenta la cantidad de libros en general y de cada categoría.
- Violacion al Open/Closed principle. Por como se manejan los generos de los libros, si quisieramos agreagar otra categoría tendríamos que modificar el código ya existente. Particularmente en el método *calcular_popularidad()*

## Soluciones aplicadas

La mejora más significativa al código es el uso de la herencia y OOP para manejar los libros de cada género. Esta estructura permite que el código sea escalable, puesto que, cuando se requiera incorporar un libro con un nuevo género basta con crear otra clase que herede de *Book* siendo esta el objeto base o "padre". Adicionalmente, reduce la repetición de código innecesaria.

Por otro lado, se modificó la distribución de las responsabilidades en cada clase. Por ejemplo, la clase *Book* ahora solo se encarga de lógica de datos internos y no es responsable de imprimirse a sí mismo. Este deber se trasladó a la clase *Library*. Además, se añadió una clase *Report* que se encarga de generar el reporte de los libros registrados en la librería.

Finalmente, se modularizaron ciertos métodos como *generar_reporte()*. Dentro de este ya no se cuentan los libros viejos o los disponibles. En su lugar, llama a las dos funciones designadas para esa tarea en especifico. 

Las principios que se tomaron en cuenta para estas mejoras fueron:

- S.O.L.I.D: Particularmente en **Single Responsability**, **Liskov Substitution** y **Open/Closed Principle**.
- KISS: Se tomó en cuenta al moduralizar los métodos.
- DRY: Se consideró a la hora de manejar los libros dependiendo de su género.
- OOP: Se hizo uso de la herencia para el manejo de los libros.