# regresion cubica
es un proceso mediante el cual, dado un conjunto de pares de puntos (xi, yi), se calculan los coeficientes de una funcion matematica cubica f()
`y = ax^3 + bx^2 + cx + d`

## pasos
1. realizar una tabla 
`x|y|x^2|x^3|x^4|x^5|x^6|xy|x^2y|x^3y`
2. sumar cada columna
`sum(x)|sum(y)|sum(x^2)|sum(x^3)|sum(x^4)|sum(x^5)|sum(x^6)|sum(xy)|sum(x^2y)|sum(x^3y)`
3. con `n = numero de datos`, generar un sistema de 4x4 ecuaciones, sustituyendo los valores de las sumatorias del paso 2
```
a*sum(xi^3) + b * sum(xi^2) + c + nd = sum(yi)                                      (ecuacion 1)
a*sum(xi^4) + b * sum(xi^3) + c * sum(xi^2) + d * sum(xi) = sum(xi * yi)            (ecuacion 2)
a*sum(xi^5) + b * sum(xi^4) + c * sum(xi^3) + d * sum(xi^2) = sum(xi ^ 2 * yi)      (ecuacion 3)
a*sum(xi^6) + b * sum(xi^5) + c * sum(xi^4) + d * sum(xi^3) = sum(xi ^ 3 * yi)      (ecuacion 4)
```
4. resolver el sistema de ecuaciones y encontrar `a`, `b`, `c` y `d`
5. sustituir `a`, `b`, `c` y `d` en la ecuacion `y = ax^3 + bx^2 + cx + d`
6. calcular estimacion del modelo, asi como el coeficiente de determinacion `R^2`

# ejemplo
obtener los valores de `x` y `y`
| x | y |
| - | - |
| -2 | 3 |
| -1 | 0 |
| -0 | 2 |
| 1 | 4 |
| 2 | 4 |

## paso 1: realizar una tabla
| x | y | x^2 | x^3 | x^4 | x^5 | x^6 | xy | x^2y | x^3y |
| - | - | --- | --- | --- | --- | --- | --- | --- | ---- |
| -2 | 3 | 4 |  -8  | 16  | -32 | 64  | -6 | 12   | -24  |
| -1 | 0 |1  | -1   |1    | -1  | 1   | 0  | 0    | 0    |
| -0 | 2 | 0  | 0   | 0   |0    | 0   | 0  | 0    | 0    |
| 1 | 4 |1  | 1   |1    | 1  | 1   | 4  | 4    | 4    |
| 2 | 4 |4  | 8   |16    | 32  | 64   | 8  | 16    | 32    |

## paso 2: sumar cada columna
|sum(x)|sum(y)|sum(x^2)|sum(x^3)|sum(x^4)|sum(x^5)|sum(x^6)|sum(x * y)|sum(x^2 * y)|sum(x^3 * y)|
| - | - | --- | --- | --- | --- | --- | --- | --- | ---- |
|0  |13|10|0|34|0|130|6|32|12|

## paso 3: con `n` numero de datos, generar un sistema de 4x4 ecuaciones, sustituyendo los valores de las sumatorias del paso 2 en:
n aqui son 5, por la cantidad de datos que proporciono el ejercicio al principio
```
a*sum(xi^3) + b * sum(xi^2) + c + nd = sum(yi)                                      (ecuacion 1)
a*sum(xi^4) + b * sum(xi^3) + c * sum(xi^2) + d * sum(xi) = sum(xi * yi)            (ecuacion 2)
a*sum(xi^5) + b * sum(xi^4) + c * sum(xi^3) + d * sum(xi^2) = sum(xi ^ 2 * yi)      (ecuacion 3)
a*sum(xi^6) + b * sum(xi^5) + c * sum(xi^4) + d * sum(xi^3) = sum(xi ^ 3 * yi)      (ecuacion 4)
```
```
0a + 10b + c + 5d = 13
34a + 0b + 10c + 0d = 6
0a + 34b + 0c + 10d = 32
130a + 0b + 34c + 0d = 12
```

## paso 4: resolver el sistema de ecuaciones
este paso me lo voy a adelantar, pero lo puedes hacer con cualquier metodo de ecuaciones lineales (montante, gauss-jordan, eliminacion gaussiana, gauss-seidel o jacobi)
```
a = -0.583
b = 0.429
c = 2.583
d = 1.743
```

## paso 5: sustituir `a`, `b`, `c` y `d` en la ecuacion `y = ax^3 + bx^2 + cx + d`
`y = -0.583x^3 + 0.429x^2 + 2.583x + 1.743`
