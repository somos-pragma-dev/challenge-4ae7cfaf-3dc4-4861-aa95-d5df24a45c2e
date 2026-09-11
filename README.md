# Desarrollo de API REST en Rust con Actix Web y Diesel ORM

El equipo de desarrollo de una fintech necesita implementar una API REST que gestione las transacciones de los usuarios. La API debe permitir la creación, lectura, actualización y eliminación de transacciones. Además, debe garantizar la idempotencia de las operaciones y manejar adecuadamente los errores del dominio.

## Informacion General

| Campo | Valor |
|-------|-------|
| **Tema** | rust-actix-web |
| **Nivel** | junior-l2 |
| **Tipo** | practical |
| **Tiempo estimado** | 8 horas |

## Fases del Reto

### Fase 0: Configuración del Proyecto

**Objetivo:** Obtener el proyecto base funcional enviando el Código Base a un asistente de IA, que lo analizará, corregirá errores y generará un ZIP listo para usar.

**Tiempo estimado:** 15-30 minutos

**Instrucciones:**

- Asegúrate de tener instalado para ejecutar el proyecto: Un IDE o editor de código.
- Copia todo el contenido del campo **Código Base** de este reto — incluyendo el texto de instrucciones que aparece al inicio.
- Abre un asistente de IA (Claude en claude.ai, ChatGPT o Gemini — se recomienda Claude), pega el contenido copiado en el chat y envíalo.
- El asistente analizará los archivos, corregirá errores y generará un archivo ZIP descargable. Descárgalo y extráelo en la carpeta donde quieras trabajar.
- Verifica que el proyecto arranca sin errores.

**Entregable:** El proyecto compila/arranca sin errores.

<details>
<summary>Pistas de conocimiento</summary>

- Copia el Código Base completo incluyendo el texto de instrucciones al inicio — esas instrucciones le indican al asistente exactamente qué hacer con los archivos.
- Si el asistente no genera el ZIP automáticamente al terminar el análisis, escríbele: "genera el ZIP ahora".
- Si el proyecto tiene errores al arrancar, comparte el mensaje de error con el mismo asistente para que lo corrija.

</details>

### Fase 1: Definición del dominio y modelado de datos

**Objetivo:** Definir las entidades y relaciones del dominio, y modelar las transacciones.

**Tiempo estimado:** 2 horas

**Instrucciones:**

- Identificar las entidades y relaciones del dominio.
- Modelar las transacciones, incluyendo los atributos necesarios y las restricciones de negocio.

**Entregable:** Modelo de datos de las transacciones.

<details>
<summary>Pistas de conocimiento</summary>

- Considera los atributos necesarios para una transacción (ID, usuario, monto, fecha, estado).
- Piensa en las restricciones de negocio (montos positivos, fechas válidas).

</details>

### Fase 2: Implementación de la API REST

**Objetivo:** Implementar los endpoints de la API REST para gestionar las transacciones.

**Tiempo estimado:** 4 horas

**Instrucciones:**

- Crear los endpoints para crear, leer, actualizar y eliminar transacciones.
- Garantizar la idempotencia de las operaciones.
- Manejar adecuadamente los errores del dominio.

**Entregable:** API REST con endpoints funcionales para gestionar transacciones.

<details>
<summary>Pistas de conocimiento</summary>

- Utiliza Actix Web para crear los endpoints.
- Implementa la idempotencia utilizando una clave única para cada operación.
- Maneja los errores del dominio de manera adecuada, proporcionando respuestas claras y útiles.

</details>

### Fase 3: Pruebas y optimización

**Objetivo:** Realizar pruebas unitarias y de integración, y optimizar el rendimiento de la API.

**Tiempo estimado:** 2 horas

**Instrucciones:**

- Escribir pruebas unitarias para los endpoints de la API.
- Realizar pruebas de integración para validar el flujo completo de las transacciones.
- Identificar y optimizar los puntos de rendimiento críticos.

**Entregable:** API REST con pruebas unitarias y de integración, y rendimiento optimizado.

<details>
<summary>Pistas de conocimiento</summary>

- Utiliza herramientas de pruebas para escribir pruebas unitarias y de integración.
- Identifica los puntos de rendimiento críticos y aplica técnicas de optimización adecuadas.

</details>

## Dimensiones Evaluadas

- **queEs**: ¿Qué es una transacción en el dominio de la fintech?
- **paraQueSirve**: ¿Para qué sirve la idempotencia en las operaciones de la API?
- **comoSeUsa**: ¿Cómo se usa Actix Web para crear endpoints en Rust?
- **erroresComunes**: ¿Cuáles son los errores comunes en el dominio de las transacciones y cómo se manejan en la API?
- **queDecisionesImplica**: ¿Qué decisiones implica la implementación de una API REST en Rust con Actix Web y Diesel ORM?

## Criterios de Evaluacion

- Definición clara del dominio y modelado de datos.
- Implementación funcional de los endpoints de la API REST.
- Garantía de idempotencia en las operaciones.
- Manejo adecuado de los errores del dominio.
- Pruebas unitarias y de integración realizadas.
- Optimización del rendimiento de la API.

## Como trabajar con un asistente de IA

- **AGENTS.md** — instrucciones nativas del repo (Cursor, Codex, Copilot, Gemini, Claude Code). Abrí el proyecto y el agente las carga solo.
- **PROMPT_MEJORA.md** — el mismo prompt, para copiar y pegar en un chat (claude.ai, ChatGPT, etc.).

---

*Reto generado automaticamente por Challenge Generator - Pragma*
