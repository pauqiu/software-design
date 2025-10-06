# Tarea 3 -Factory Async

**Estudiante:** Paulette Zamora Hernández - C38632

**Diseño de Software**

## Diseño de la solución

### FoodService

Esta clase se encarga de administrar a los hilos que procesarán las ordenes.

Para repartir las ordenes en la lista se utiliza asignación dinámica. 
Esto se logra a través de una _queue_. La librería _queue_ provee una estructura
 de datos _thread_safe_ que evita condiciones de carrera a 
la hora de acceder a items dentro de la misma. Una vez un hilo ejecuta 
_get()_, obtiene una orden y la procesa. Si el método _get()_ no devuelve
ninguna orden, el hilo sale de _process_orders()_ para terminar su ejecución.

El método de _queue.join()_ espera a que todas las tareas hayan sido 
procesadas. Lo que hace internamente es comparar el número _task_done()_
con el número de _get()_ ejecutados. Cuando los números coinciden se dan las
tareas por terminadas y se deja a los hilos pasar a terminar su ejecución.

Cabe destacar que para tareas más complejas puede que sea necesario agregar 
una señal de finalización propiamente de los hilos y no contar solamente con
que la _queue_ esté vacía para finalizar la ejecución.

### PizzaFactory & HamburgerFactory

Simuladores de fábricas de hamburguesas y pizzas. Cada archivo también cuenta 
con una clase _Pizza_ o _Hamburguesa_ según le corresponda. Estas respresentan
las verdaderas ordenes a procesar.

Dentro de las fábricas se cuenta con _locks_ que permiten la impresión correcta
de las líneas de _debug_.

### OrderWrapper

Esta clase se encarga de encapsular las ordenes con su fábrica correspondiente
para qua, a la hora de procesarlas no se necesite conocer su tipo. De este modo
se ahorran las líneas de distinción de tipo y simplemente se llama a
_make_order()_ para cualquiera de los casos.

### Link al repositorio

    https://github.com/pauqiu/software-design/tree/main/homework3