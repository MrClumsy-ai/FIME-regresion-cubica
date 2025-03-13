# Regresion cubica

Es un proceso mediante el cual, dado un conjunto de pares de puntos `(xi, yi)`, se calculan los coeficientes de una funcion matematica cubica `f(x)`.
`y = ax³ + bx² + cx + d`

## Pasos

1. Realizar una tabla .
   `| x | y | x² | x³ | x⁴ | x⁵ | x⁶ | xy | x²y | x³y |`

2. Sumar cada columna.
   `| sum(x) | sum(y) | sum(x²) | sum(x³) | sum(x⁴) | sum(x⁵) | sum(x⁶) | sum(xy) | sum(x²y) | sum(x³y) |`

3. Con `n = numero de datos`, generar un sistema de 4x4 ecuaciones, sustituyendo los valores de las sumatorias del paso 2.

```
sum(x³)a + sum(x²)b + sum(x)c  + nd       = sum(y)    (ecuacion 1)
sum(x⁴)a + sum(x³)b + sum(x²)c + sum(x)d  = sum(xy)   (ecuacion 2)
sum(x⁵)a + sum(x⁴)b + sum(x³)c + sum(x²)d = sum(x²y)  (ecuacion 3)
sum(x⁶)a + sum(x⁵)b + sum(x⁴)c + sum(x³)d = sum(x³y)  (ecuacion 4)
```

4. Resolver el sistema de ecuaciones y encontrar `a`, `b`, `c` y `d`.

5. Sustituir `a`, `b`, `c` y `d` en la ecuacion `y = ax³ + bx² + cx + d`.

6. Calcular estimacion del modelo, asi como el coeficiente de determinacion `R²`.

# Ejemplo

Obtener los valores de `x` y `y`.
|x|y|
|-|-|
|-2|3|
|-1|0|
|-0|2|
|1|4|
|2|4|

## Paso 1: Realizar una tabla

| x   | y   | x²  | x³  | x⁴  | x⁵  | x⁶  | xy  | x²y | x³y |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| -2  | 3   | 4   | -8  | 16  | -32 | 64  | -6  | 12  | -24 |
| -1  | 0   | 1   | -1  | 1   | -1  | 1   | 0   | 0   | 0   |
| -0  | 2   | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
| 1   | 4   | 1   | 1   | 1   | 1   | 1   | 4   | 4   | 4   |
| 2   | 4   | 4   | 8   | 16  | 32  | 64  | 8   | 16  | 32  |

## Paso 2: Sumar cada columna

| sum(x) | sum(y) | sum(x²) | sum(x³) | sum(x⁴) | sum(x⁵) | sum(x⁶) | sum(xy) | sum(x²y) | sum(x³y) |
| ------ | ------ | ------- | ------- | ------- | ------- | ------- | ------- | -------- | -------- |
| 0      | 13     | 10      | 0       | 34      | 0       | 130     | 6       | 32       | 12       |

## Paso 3: Con `n` numero de datos, generar un sistema de 4x4 ecuaciones, sustituyendo los valores de las sumatorias del paso 2 en:

```
sum(x³)a + sum(x²)b + sum(x)c  + nd       = sum(y)    (ecuacion 1)
sum(x⁴)a + sum(x³)b + sum(x²)c + sum(x)d  = sum(xy)   (ecuacion 2)
sum(x⁵)a + sum(x⁴)b + sum(x³)c + sum(x²)d = sum(x²y)  (ecuacion 3)
sum(x⁶)a + sum(x⁵)b + sum(x⁴)c + sum(x³)d = sum(x³y)  (ecuacion 4)
```

`n` aqui son 5, por la cantidad de datos que proporciono el ejercicio al principio.

```
0a   + 10b + 0c  + 5d  = 13
34a  + 0b  + 10c + 0d  = 6
0a   + 34b + 0c  + 10d = 32
130a + 0b  + 34c + 0d  = 12
```

## Paso 4: resolver el sistema de ecuaciones

Este paso me lo voy a adelantar, pero lo puedes hacer con cualquier metodo de ecuaciones lineales (montante, gauss-jordan, eliminacion gaussiana, gauss-seidel o jacobi).

```
a = -0.583
b = 0.429
c = 2.583
d = 1.743
```

## Paso 5: Sustituir `a`, `b`, `c` y `d` en la ecuacion `y = ax³ + bx² + cx + d`

`y = -0.583x³ + 0.429x² + 2.583x + 1.743`
