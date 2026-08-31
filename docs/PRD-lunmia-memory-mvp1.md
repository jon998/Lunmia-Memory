# Lunmia Memory — PRD MVP 1

**Versión:** 1.0
**Fecha:** agosto 2026
**Autor:** Yonatan
**Estado:** decisiones de producto cerradas; pendientes tres puntos marcados en §14

> Este documento reemplaza por completo a `PRD-nexus-mvp1.md` (v0.2). El producto anterior, *Nexus Orchestrator*, queda descartado: no es un cambio de nombre, es un cambio de problema.

---

## 1. Resumen ejecutivo

Lunmia Memory es una aplicación de escritorio para macOS que captura ideas, notas, recordatorios y capturas de pantalla en menos de tres segundos, y las clasifica automáticamente usando modelos de IA que corren en la máquina del usuario.

La promesa no es organizar mejor. Es que **nada se pierda**.

El diferenciador no es el modelo de lenguaje —eso es infraestructura barata y comoditizada— sino tres cosas que las herramientas existentes no combinan: captura sin fricción disponible en cualquier momento sobre cualquier aplicación, procesamiento local sin enviar nada a la nube, y una taxonomía que el usuario posee y que evoluciona con su uso real en lugar de venir impuesta desde el primer día.

---

## 2. El problema

El autor pierde información de forma sistemática por cinco vías distintas:

1. **Ideas que no se anotan.** Aparecen en momentos malos —caminando, en una reunión, a mitad de otra tarea— y anotarlas cuesta más que el valor aparente de la idea en ese instante.
2. **Notas que se anotan pero no se clasifican.** Terminan en un archivo, una nota suelta o un mensaje a sí mismo, y no vuelven a aparecer nunca.
3. **Recomendaciones recibidas en conversación.** Un libro, una herramienta, un contacto. Se olvidan en horas.
4. **Seguimiento con equipos.** Compromisos adquiridos en reuniones que no llegan a ningún sistema.
5. **Información visual.** Una captura de pantalla con una fecha, un dato, una petición. Se guarda en Descargas y muere ahí.

El patrón común no es falta de herramientas. Es que **el costo de capturar en el momento exacto en que la información existe es más alto que cero**, y toda información que exige un ritual para entrar al sistema termina fuera del sistema.

### 2.1 Por qué no basta con lo que ya existe

| Herramienta | Concepto valioso | Por qué no resuelve |
|---|---|---|
| **Tana** | Supertags: clasificar al capturar convierte notas en base de datos consultable | Solo nube, sin modo offline real; configuración inicial pesada |
| **Saner.ai** | Organización automática, búsqueda semántica, síntesis | Sin app nativa de Mac (web + extensión + iOS en Apple Silicon) |
| **AudioPen / Voicenotes** | Fricción de captura por voz casi nula | Solo voz; sin clasificación estructurada ni recordatorios |
| **Obsidian / Logseq** | Local-first, control total del dato | La organización es trabajo manual del usuario |
| **Akiflow / Routine** | Consolidación de tareas y calendario | Cloud-first; no capturan ideas sin estructura |

**El hueco:** nadie combina local-first + IA local + atajo nativo del sistema operativo + un mismo inbox para lo personal y lo profesional.

---

## 3. Objetivos y no-objetivos

### 3.1 Objetivos del MVP 1

- Capturar texto, voz o captura de pantalla en menos de 3 segundos desde cualquier aplicación.
- Clasificar automáticamente cada Entrada por tipo, espacio y proyecto sin intervención del usuario en el caso normal.
- Extraer fechas y generar recordatorios que **disparan con la aplicación cerrada**.
- Permitir corregir cualquier clasificación errónea con una sola tecla.
- Funcionar completamente sin conexión a internet.
- Que la taxonomía crezca desde el uso real del usuario, no desde una lista predefinida.

### 3.2 No-objetivos explícitos

- **No sincronizar con calendarios externos en esta fase.** (Reformulación deliberada del antiguo "no reemplazar el calendario del usuario", que era ambiguo.)
- **No ser multiplataforma en el MVP 1.** Solo macOS.
- **No ser un gestor de tareas ni un sistema de proyectos.** No hay dependencias, asignaciones, ni estados de flujo de trabajo.
- **No almacenar imágenes.** Ver §6.4.
- **No integrar Git, MCP ni Notion.** Ver §15.2.
- **No perseguir precisión máxima de clasificación.** Ver §12.1.

---

## 4. Usuario

**Usuario único del MVP 1: el autor.**

Esto no es una limitación temporal, es una decisión de método. Un producto de captura personal que no resuelve el problema de su autor no lo va a resolver para nadie más, y validar con desconocidos antes de validar consigo mismo introduce ruido que no se puede interpretar.

Perfil relevante para decisiones técnicas:

- Escribe mezclando español e inglés en la misma frase. **Consecuencia dura: todo modelo de embeddings debe ser multilingüe.**
- Trabaja en macOS con chip Apple Silicon.
- Alterna entre trabajo, proyectos personales y estudio a lo largo del mismo día.
- Es desarrollador, lo que significa tolerancia alta a configuración pero cero tolerancia a lentitud.

---

## 5. Modelo de datos

### 5.1 Jerarquía

```
Usuario
  ├── Taxonomía (tipos de Entrada, propiedad del usuario, evolutiva)
  ├── Etiquetas (libres, sin gobernanza)
  ├── Contexto activo (sesión de trabajo aprendida)
  ├── Vista "Todos los calendarios"   ← derivada, no es entidad
  └── Espacio (trabajo | personal | cliente | estudio)
        ├── Calendario (exactamente 1, UNIQUE por espacio_id)
        │     └── Fuente externa (0..n)   ← FASE 2
        ├── Proyecto (uno por defecto, no borrable)
        │     └── Entrada
        │           └── Pregunta pendiente (0..2)
        └── Colección (vista generada por IA, no destructiva)
```

### 5.2 Reglas estructurales

**Un Espacio tiene exactamente un calendario.** "Todos los calendarios" es una consulta agregada, no una entidad. Al crear un evento desde esa vista, el sistema exige elegir Espacio.

**"Personal" no es un caso especial.** Es un Espacio de tipo `personal`. Un solo concepto, múltiples instancias: menos código, menos ramas condicionales, menos deuda.

**Nada generado por IA es destructivo.** Las Colecciones son vistas, no carpetas. Tienen un botón "Convertir en proyecto" para que el usuario promueva lo que le sirva. La IA propone; el usuario decide.

**Espacio y Proyecto son obligatorios en toda Entrada.** De ahí se deriva que cada Espacio nazca con un proyecto por defecto no borrable. Regla de agotamiento: si la IA no determina proyecto y el presupuesto diario de preguntas ya se gastó, la Entrada cae al proyecto por defecto marcada como **provisional**, y el resurfacing la prioriza.

> **Señal de alarma:** si más del 70% de las Entradas vive en proyectos por defecto, la clasificación por proyecto no funciona y hay que rediseñarla, no ajustar el umbral.

### 5.3 La Entrada

`Entrada` es la unidad atómica del sistema. Tabla `entradas`, clave `entry_id`, enumeración `EntryType`.

El nombre importa: deja libre la palabra "captura" para el **acto** de capturar y para "captura de pantalla", que era una colisión de vocabulario real en la versión anterior.

| Campo | Tipo | Notas |
|---|---|---|
| `entry_id` | UUIDv7 | Ver §5.6 |
| `contenido_original` | texto | **Inmutable.** Se escribe una vez y nunca se toca |
| `contenido` | texto | Copia inicial de `contenido_original`; es lo editable, lo que se muestra y sobre lo que se busca |
| `tipo_id` | FK taxonomía | Exactamente uno |
| `espacio_id` | FK espacio | Obligatorio |
| `proyecto_id` | FK proyecto | Obligatorio; puede ser el proyecto por defecto |
| `es_provisional` | booleano | Verdadero si cayó al proyecto por defecto por agotamiento |
| `estado` | enum | `inbox \| pendiente_resolucion \| triaged \| activo \| archivado` |
| `origen` | enum | `texto \| voz \| captura_pantalla \| telegram` |
| `metadata_captura` | JSON | App en primer plano, título de ventana, timestamp |
| `confianza` | real | Confianza de la clasificación, por capa |
| `created_at` / `updated_at` | timestamp | |
| `deleted_at` | timestamp nullable | Tombstone, ver §5.6 |

**Original inmutable + campo editable, uniforme para todos los orígenes.** `contenido_original` guarda lo que entró al sistema: el texto completo del OCR, la transcripción de voz, o lo que el usuario tecleó. `contenido` nace como copia y es lo que el usuario edita.

La uniformidad es deliberada: una sola regla para todos los orígenes significa menos condicionales y menos casos especiales. Y protege el caso peligroso —la imagen ya se borró, el OCR es el único rastro— sin necesidad de razonar sobre él cada vez.

**Al editar, la reclasificación se propone, no se aplica.** Sobrescribir automáticamente destruiría las correcciones manuales del usuario, que son precisamente los ejemplos de entrenamiento de la capa 2. Sin historial de versiones en el MVP 1; basta `updated_at`.

### 5.4 Los dos ejes de clasificación

**Eje 1 — Tipo.** Un valor por Entrada. Dinámico: vive en una tabla, no en un enum del código. Cada tipo mantiene un centroide de embedding calculado a partir de sus Entradas.

Gobernanza del tipo:
- Cuando dos centroides se solapan por encima del umbral, el sistema **propone** fusionarlos.
- Cuando un tipo lleva mucho tiempo sin uso, el sistema **propone** archivarlo.
- Límite blando de 12 tipos activos. Superarlo dispara una revisión, no un bloqueo.

**Eje 2 — Etiquetas.** Varias por Entrada. Libres, sin gobernanza. La IA las propone.

La razón de que existan los dos ejes: **las etiquetas son la válvula de escape que impide la explosión de la taxonomía.** Sin ellas nacen tipos como "recomendación urgente" o "idea para revisar después", que no son tipos sino combinaciones de un tipo con un atributo. Con etiquetas, eso se expresa sin ensuciar el eje principal.

> **El sistema de preguntas asistidas nunca pregunta por etiquetas.** Solo por tipo, fecha y proyecto. Preguntar por etiquetas gastaría presupuesto de atención en el eje que menos importa.

### 5.5 Contexto activo

Concepto central, aportado por el usuario.

Al encender la Mac, o tras un periodo largo de inactividad, el sistema pregunta en qué proyecto se va a trabajar. Esa respuesta se convierte en el **contexto activo** de la sesión.

Con el tiempo aprende patrones: si de lunes a viernes entre las 7am y las 4pm el contexto es siempre Espacio Trabajo, proyecto X, deja de preguntar y lo propone.

**El aprendizaje es una tabla de frecuencias por día de semana y franja horaria. No es IA.** Es determinista, explicable y depurable. Si propone mal, se puede ver exactamente por qué en una consulta SQL.

**El contexto activo es la capa 0, y solo para Espacio y Proyecto.** El tipo se sigue derivando del contenido de la Entrada, nunca del contexto: estar trabajando no convierte una idea personal en una nota de trabajo.

### 5.6 Requisito de sincronización desde el día uno

La fase 2 va a la nube y a multiplataforma. El esquema debe estar preparado desde ahora:

- **UUIDv7 como clave primaria en todas las tablas.** No enteros autoincrementales.
- **`updated_at` en cada fila**, sin excepción.
- **Borrado suave con tombstones** (`deleted_at`), nunca `DELETE`.
- **Sin dependencias de orden de inserción.**

Costo de hacerlo hoy: aproximadamente una hora. Costo de omitirlo: una migración completa de datos con el producto ya en uso.

### 5.7 Referencia externa

Tabla genérica de pares tipo/valor (`url`, `repo`, `ticket`, `archivo`) asociable a Entrada y a Proyecto.

No se usa en el MVP 1. Existe para que la vertical futura de desarrolladores (§15.2) no exija una migración de esquema. Es una tabla vacía y unos pocos campos: el costo hoy es despreciable.

---

## 6. Pipeline de clasificación

### 6.1 Principio rector

**La capacidad del modelo no es el producto.** El producto es la captura, los recordatorios que sí disparan, y la corrección barata. La IA es aproximadamente tres días de trabajo; todo lo demás es el otro 80%.

De ahí se deriva todo lo que sigue.

### 6.2 Cascada por costo

No se usa un modelo grande. Se usa la capa más barata que resuelva el caso, y se sube solo si es necesario:

| Capa | Mecanismo | Cubre |
|---|---|---|
| **0** | Contexto activo | Espacio y Proyecto (§5.5) |
| **1** | Reglas: fechas, verbos, prefijos explícitos | La mayoría de casos con estructura |
| **2** | Embeddings + vecino más cercano contra centroides | Tipo, cuando hay historial |
| **3** | LLM pequeño (1–3B parámetros) | Casos ambiguos y extracción compleja |

**La mayoría de las tareas "de IA" no necesitan IA.** Cada caso resuelto en la capa 1 es un caso instantáneo, gratis y explicable.

### 6.3 La clasificación nunca está en el camino crítico

Orden obligatorio:

1. Guardar en SQLite
2. Confirmar al usuario
3. Clasificar en segundo plano

El usuario nunca espera al modelo. Si el modelo falla, la Entrada ya está guardada y cae a `inbox`.

### 6.4 Capturas de pantalla y OCR

**Las imágenes se procesan y se descartan inmediatamente. Nunca se almacenan.**

Flujo objetivo: el usuario toma una captura y dice "ayúdame a recordar esto". El sistema extrae el texto, detecta si es una reunión, un dato relevante para un proyecto o una petición que validar con el equipo, y guarda.

**OCR mediante el framework Vision nativo de macOS.** Cero megabytes de descarga, aproximadamente 40ms, sin dependencias externas. Para texto digital nítido —que es el 95% de las capturas de pantalla— el OCR clásico ya está en el techo de precisión: un modelo de visión no aporta nada y alucina.

Consecuencias de descartar la imagen:
- El texto del OCR es el artefacto durable. De ahí la inmutabilidad de `contenido_original` (§5.3).
- No existe la pregunta asistida que muestra la imagen para desambiguar.
- La extracción es de un solo intento: texto completo sin truncar, más la metadata del momento.

**Cuando llegan imagen y texto del usuario juntos, la señal más fuerte para clasificar es el texto que escribió el usuario.** El OCR entra como contexto de respaldo, no como fuente principal.

### 6.5 Fechas en lenguaje natural

**Corrección importante respecto a versiones anteriores del documento:** `chrono` es una biblioteca de manejo de fechas en Rust, **no** un parser de lenguaje natural. Los parsers de lenguaje natural disponibles en Rust están hechos para inglés y su soporte de español es pobre.

Solución: reglas propias en español. Unas 30 expresiones cubren aproximadamente el 90% de los casos —hoy, mañana, pasado mañana, el lunes, la próxima semana, en X días, el 15, a fin de mes— con caída al LLM para lo que no encaje.

### 6.6 Resolución asistida

Cuando la confianza cae por debajo del umbral, la Entrada genera **hasta 2 preguntas**, con botones fijos y siempre una opción de texto libre.

Disparadores:
- Fecha ambigua o que ya pasó
- Dos tipos candidatos dentro de un margen estrecho
- Ningún proyecto supera el umbral habiendo más de uno activo
- Espacio ambiguo
- Entrada que no encaja en ningún tipo existente → propone crear uno nuevo

Reglas:
- **Nunca preguntar durante la captura.**
- Preguntas agrupadas en la bandeja, no de una en una.
- Presupuesto diario configurable, por defecto 10.
- Priorización por urgencia y fecha próxima.
- Umbral adaptativo: si el usuario descarta sistemáticamente, el sistema pregunta menos.
- Descartar siempre disponible. La Entrada vuelve a `inbox`; nunca se pierde.
- Toda respuesta alimenta la capa 2.

> **Métrica de salud:** si la tasa de descarte supera el 40%, el sistema está preguntando de más.

### 6.7 Panel de confirmación

**No bloqueante por defecto.** Enter ya guardó la Entrada. El panel aparece después con:

- El resumen de lo capturado
- Dos selectores prellenados: Espacio y Proyecto
- Un campo de prompt debajo para corregir en lenguaje natural

Se cierra solo a los pocos segundos, guardando lo propuesto.

**Excepción: en capturas de pantalla el panel sí espera confirmación**, porque la imagen ya se borró y no hay segunda oportunidad.

El campo de prompt es una alternativa a los selectores, no la vía principal: cuesta una segunda llamada al modelo.

---

## 7. Presencia y sistema de diseño

El principio: **la herramienta tiene que estar donde ocurre la idea, no donde está la ventana de la aplicación.** Una app que exige cambiar de ventana ya perdió.

### 7.1 Las tres piezas

**1. Icono permanente en la barra de menú.** Siempre visible, siempre accesible. Muestra el contexto activo actual y el número de Entradas sin triage.

**2. Ventana flotante que aparece sobre cualquier aplicación sin robar el foco.** Ventana de nivel superior, sin decoración, centrada. Aparece, recibe la captura, desaparece. El usuario nunca sale de lo que estaba haciendo.

**3. Atajo global del sistema.**

### 7.2 Sobre el atajo

**`Cmd + Space` está tomado por Spotlight y esa es una guerra que no se puede ganar.** Redefinirlo genera conflictos, confunde al usuario y rompe una expectativa de veinte años.

Recomendación: **`Cmd + Shift + Space`**. Suele estar libre y respeta el gesto mental que se busca.

Alternativa: doble pulsación de una tecla modificadora, patrón usado por varias herramientas de captura rápida en Mac. Más elegante, más difícil de implementar bien.

El atajo debe ser configurable desde el primer día.

### 7.3 Sobre el notch

**macOS no ofrece ninguna API pública para dibujar en el notch.** Las aplicaciones que parecen hacerlo dibujan una ventana flotante justo debajo, simulando el efecto.

Es viable, pero es un truco visual y suma un trabajo de interfaz considerable para un beneficio estético.

**Decisión: fuera del MVP 1.** La ventana flotante centrada resuelve el 100% de la necesidad funcional. El notch es refinamiento posterior.

### 7.4 Requisito arquitectónico

Toda esta capa es específica de macOS: barra de menú, atajo global, ventana flotante, notificaciones, autostart, OCR, voz.

**Todo detrás de una capa de adaptadores (traits en Rust).** El núcleo no debe saber en qué sistema operativo corre. Sin esto, la multiplataforma de la fase 2 es una reescritura.

---

## 8. Onboarding generativo

No se presenta al usuario una lista de tipos predefinidos. Se le pregunta a qué se dedica, y el modelo **propone entre 5 y 7 tipos adaptados a su realidad**, que el usuario acepta, renombra o descarta.

Por qué: una taxonomía impuesta se abandona; una taxonomía propuesta y editada se adopta. Y el acto de editarla es la primera señal de entrenamiento que recibe el sistema.

Requisitos del onboarding:
- No debe bloquearse esperando descargas de modelos. Ver §9.
- Debe terminar en menos de 2 minutos.
- Debe terminar con **una captura real hecha por el usuario**, no con una pantalla de "todo listo".

---

## 9. Gestión de modelos locales

**Cola con un único worker** (`mpsc` + mutex) para serializar las inferencias. Lo que mata el rendimiento no es tener modelos instalados, es ejecutarlos en paralelo.

**`keep_alive` diferenciado:**

| Modelo | keep_alive | Razón |
|---|---|---|
| Embeddings | infinito | Se usa constantemente, es pequeño |
| Clasificador | 5 minutos | Uso intermitente |

**Embeddings multilingües obligatorios.** El usuario escribe mezclando español e inglés. Candidatos: `bge-m3`, `snowflake-arctic-embed2`. Muchos modelos pequeños de embeddings son solo inglés y degradarían silenciosamente la capa 2 — silenciosamente es la palabra peligrosa.

**Perfiles automáticos por RAM detectada** (Ligero / Equilibrado / Completo). **Nunca se le pregunta al usuario qué modelo quiere.** Esa pregunta no tiene respuesta buena para nadie que no sea ingeniero de machine learning, y aún para ellos es una molestia.

---

## 10. Alcance funcional del MVP 1

### Dentro

- Captura por texto con atajo global
- Captura de pantalla con OCR vía Vision de macOS
- Clasificación automática en cascada por costo (capas 0–3)
- Taxonomía dinámica con fusión y archivado propuestos
- Etiquetas libres
- Espacios, proyectos, contexto activo
- Recordatorios locales que disparan con la app cerrada
- Bandeja con resolución asistida
- Búsqueda semántica sobre `contenido`
- Colecciones generadas por IA (vistas no destructivas)
- Onboarding generativo

### Fuera

- Multiplataforma
- Sincronización con calendarios externos
- Sincronización en la nube entre dispositivos
- Almacenamiento de imágenes
- Modelos de visión (VLM)
- Git, MCP, Notion
- Notch
- Colaboración o compartición

---

## 11. Stack técnico

| Capa | Elección | Notas |
|---|---|---|
| Shell de escritorio | Tauri 2.0 | Binario pequeño, buen soporte de APIs nativas |
| Backend | Rust | Adaptadores de plataforma en traits (§7.4) |
| Frontend | SolidJS | Reactividad fina, sin virtual DOM |
| Persistencia | SQLite | Con extensión vectorial para embeddings |
| IA local | Ollama | Modelo pequeño 1–3B + embeddings multilingües |
| OCR | Vision (macOS) | Nativo, 0 MB, ~40ms |
| Voz | Speech (macOS) | Pendiente de decisión, ver §14 |

---

## 12. Métricas

### 12.1 Criterio rector

**La calidad la determina qué tan barato es corregir a la IA, no qué tan bien acierta.**

Un sistema con 95% de precisión donde corregir cuesta cinco clics es peor que uno con 80% donde corregir cuesta una tecla. Todas las decisiones de diseño se subordinan a esto.

### 12.2 Métrica primaria

**El autor usa la aplicación 21 días consecutivos y el inbox nunca supera 20 elementos sin triage.**

Una sola métrica, binaria, sin margen de interpretación favorable.

### 12.3 Métricas de apoyo

- Latencia de captura por debajo de 3 segundos
- Más del 80% de precisión de clasificación tras 200 Entradas
- Corrección en una sola tecla
- 100% de recordatorios disparados con la aplicación cerrada
- 0 Entradas perdidas
- Tasa de descarte de preguntas por debajo del 40% (§6.6)
- Menos del 70% de Entradas en proyectos por defecto (§5.2)

### 12.4 Instrumentación

**Instrumentar desde la primera línea de código:** latencia por operación, confianza por capa, tasa de descarte, aciertos y correcciones.

Sin esos números, todas las decisiones de la fase 2 serán opinión disfrazada de criterio.

---

## 13. Riesgos

| ID | Riesgo | Severidad | Mitigación |
|---|---|---|---|
| **R1** | Los recordatorios no disparan con la app cerrada | **Crítica** | **Prototipar el scheduler antes que cualquier otro código.** Si esto no funciona, el producto no existe |
| **R2** | La captura no está donde ocurren las ideas (fuera de la Mac) | Alta | Bot de Telegram como canal móvil (§14) |
| **R3** | Sobreinversión en la capa de IA | Alta | Presupuesto explícito: ~3 días. Si se excede, parar |
| **R4** | Onboarding bloqueado por descarga de modelos | Alta | Perfiles automáticos; primera captura funcional antes de que termine la descarga |
| **R5** | La taxonomía explota y deja de ser útil | Media | Dos ejes (§5.4), fusión de centroides, límite blando de 12 |
| **R6** | Esquema no preparado para sincronización | Media | §5.6, resuelto |
| **R7** | Inferencias concurrentes degradan el sistema | Media | Cola de un worker (§9) |

---

## 14. Decisiones pendientes

Ninguna bloquea el inicio del desarrollo.

**D1 — Captura por voz en el MVP 1.**
Recomendación: sí, con el framework **Speech nativo de macOS**: offline, en dispositivo, soporta español, cero descarga. Frente a Whisper local, que exige unos 500 MB para funcionar decentemente en español/inglés mezclados.
Limitaciones honestas del nativo: peor con jerga técnica y nombres propios, topes de duración por solicitud, posible descarga del paquete de idioma. Adecuado para capturas de 10 a 20 segundos, que es el caso de uso real.

**D2 — Bot de Telegram como canal de captura móvil.**
Cierra el riesgo R2, que es el segundo más grave del documento. No requiere servidor propio: la aplicación de escritorio hace long polling, y Telegram retiene los mensajes hasta 24 horas, así que se descargan al abrir la app. Aproximadamente 200–300 líneas de Rust.
Recomendación: sí, limitado a texto.

**D3 — Self-host frente a nube gestionada para la fase 2.**
Afecta al modelo de negocio, a la privacidad y al costo. Debe decidirse **antes** de escribir la capa de sincronización, no después.

**D4 — Aviso cuando el OCR no extrae texto útil.**
Excepción propuesta a la regla de no interrumpir: avisar en el momento ("no pude leer texto en esta imagen, ¿la describes?"). Sin esto hay pérdida silenciosa de datos, que es la peor clase de pérdida.

---

## 15. Roadmap

### 15.1 Fases

| Fase | Contenido | Criterio de salida |
|---|---|---|
| **0 — Prototipo de riesgo** | Solo el scheduler de recordatorios con la app cerrada | Un recordatorio dispara de forma fiable 20 veces seguidas |
| **1 — Captura** | Atajo global, ventana flotante, persistencia, barra de menú | Captura en menos de 3s |
| **2 — Clasificación** | Cascada de capas 0–3, taxonomía, contexto activo | 80% de precisión tras 200 Entradas |
| **3 — Bandeja** | Resolución asistida, búsqueda, colecciones | Tasa de descarte bajo 40% |
| **4 — Pulido** | Onboarding generativo, perfiles de modelo, instrumentación | 21 días de uso continuo |

**La fase 0 es innegociable y va primero.** El riesgo R1 es existencial: si los recordatorios no disparan con la aplicación cerrada, no hay producto que construir, y descubrirlo en la semana ocho sería catastrófico.

### 15.2 Fuera del roadmap

El roadmap heredado de Nexus Orchestrator queda **eliminado**: fuera Git con worktrees, fuera MCP y Notion.

La capa para desarrolladores —asociar proyectos con repositorios— queda como **visión sin fecha ni alcance definido**, no como fase planificada. La tabla de referencia externa (§5.7) existe para que esa vertical, si algún día llega, no exija migración.

---

## 16. Validación previa recomendada

**Antes de escribir código: dos semanas capturando todo en un archivo de texto plano o un bot de Telegram, sin aplicación.**

Qué produce, a costo casi cero:
- Cuántas capturas reales por día
- Qué proporción lleva fecha
- Qué tipos emergen de verdad, frente a los que se imaginan en abstracto
- Qué porcentaje es imagen-dominante

Y algo más valioso: **ese corpus es el conjunto de prueba del clasificador.** Sin él, la precisión del 80% de §12.3 no se puede medir contra nada.

---

## 17. Notas docentes

Este proyecto sirve como caso de estudio para el curso planeado. Los momentos con valor pedagógico:

- **El reencuadre.** Nexus Orchestrator era una solución buscando un problema. El cambio a Lunmia Memory ocurrió al describir el problema real en voz alta.
- **La IA como el módulo barato.** Tres días de doce semanas. Contraintuitivo y central.
- **La cascada por costo.** La mayoría de las tareas "de IA" no necesitan IA.
- **El determinismo donde se puede.** El contexto activo es una tabla de frecuencias, no un modelo: explicable y depurable.
- **Nada generado por IA es destructivo.** La IA propone; el usuario decide.
- **Prototipar el riesgo existencial primero,** no lo entretenido.
- **La corrección barata vence a la precisión alta.**

---

*Fin del documento.*
