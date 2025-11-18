# Tarea 4 - Patrones para cafetería

**Estudiante:** Paulette Zamora Hernández - C38632

**Diseño de Software**

## Patrones elegidos

Para resolver este problema se seleccionaron los patrones de diseño  `Observer` y  `Decorator`

# 1. Patrón **Decorator**

Se utilizó para manejar las variaciones de los pedidos de los clientes:

- Cada bebida o alimento base (Café, Té, Croissant, Muffin…) actúa como un componente principal.
- Cada ingrediente extra (leche, crema, canela, toppings, rellenos) se modela como un **decorador**.
- Los decoradores envuelven al producto base y modifican sus componentes.

### Ventajas concretas para este caso:

1. **Combinación ilimitada de personalizaciones**  
   El cliente puede pedir café con leche, o café con leche y canela, o café con leche, canela y crema.  
   Cada extra se envuelve uno encima del otro sin explosionar el número de clases.

2. **Evita herencia innecesaria**  
   No se necesita crear clases como `CafeConLeche`, `CafeConLecheYCrema`, `CafeConLecheCremaCanela`, etc.  

3. **Nuevos toppings sin tocar código existente**  
   Si se quisieran agregar nuevos productos a la cafetería, es tan fácil como agregar decoradores o bases para integrar al sistema.
   No hace falta modificar las clases de bebidas o alimentos.

4. **Responsabilidades claras**  
   - Producto base: define la funcionalidad mínima.  
   - Decoradores: agregan funcionalidad opcional.

# 2. Patrón **Observer**

Se utilizó para notificar a los usuarios cuando un pedido está listo. Tal y como en una cafetería real, el observador anuncia el nombre correspondiente al pedido listo.

- El **pedido** es el sujeto (Subject).
- El **dispatcher** es notificado cuando un pedido está listo para que sea anunciado a la clientela.

### Ventajas concretas:

1. **Notificaciones automáticas**
   Cuando el pedido está listo todos el observador es actualizado sin que el pedido necesite conocer su implementación.

2. **El sistema crece sin romper nada**
   Añadir un nuevo observador no requiere modificar la clase `Order`.  
   Basta con registrarlo para que reciba eventos.

3. **Separación clara de responsabilidades**
   - El pedido solo sabe que debe notificar.
   - Los observadores deciden cómo reaccionar.

# Link al repositorio

https://github.com/pauqiu/software-design

---
