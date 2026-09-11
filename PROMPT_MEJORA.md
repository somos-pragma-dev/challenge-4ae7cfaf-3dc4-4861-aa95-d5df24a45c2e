# Prompt para Mejorar el Codigo Base

Copia y pega el siguiente contenido completo en un asistente de IA (Claude, ChatGPT, etc.)
para obtener un ZIP con el proyecto arrancable. Si el adjunto es una carcasa (docs/placeholders),
el asistente debe materializar la estructura del stack del briefing, sin resolver las fases del reto.

---

```
## Briefing del reto (autoridad)
Este bloque manda sobre los archivos adjuntos. El stack y el rol salen de AQUÍ, no de un topic genérico ni de markdown placeholder.

### Contexto técnico original
Build a REST API with Rust, Actix Web and Diesel ORM

### Reto
- Tema: rust-actix-web
- Seniority: junior-l2
- Tipo: practical
- Título: Desarrollo de API REST en Rust con Actix Web y Diesel ORM
- Tiempo estimado: 8 horas

### Fases (trabajo del HUMANO — PROHIBIDO completarlas)
No implementes estos entregables. Dejalos como hueco pedagógico. El asistente solo materializa el proyecto arrancable para que el participante pueda trabajar.
- Fase 1: Definición del dominio y modelado de datos — objetivo: Definir las entidades y relaciones del dominio, y modelar las transacciones. — entregable (NO resolver): Modelo de datos de las transacciones.
- Fase 2: Implementación de la API REST — objetivo: Implementar los endpoints de la API REST para gestionar las transacciones. — entregable (NO resolver): API REST con endpoints funcionales para gestionar transacciones.
- Fase 3: Pruebas y optimización — objetivo: Realizar pruebas unitarias y de integración, y optimizar el rendimiento de la API. — entregable (NO resolver): API REST con pruebas unitarias y de integración, y rendimiento optimizado.

Eres un asistente experto en análisis, corrección y generación de archivos de cualquier tipo:
código fuente, documentación, hojas de cálculo, documentos Word, configuraciones, entre otros.
Voy a enviarte una cadena de texto que contiene uno o más archivos. Cada archivo está delimitado por un marcador con el siguiente formato:
// === ARCHIVO: ruta/del/archivo.extension ===
o también puede aparecer como:
## === ARCHIVO: ruta/del/archivo.extension ===
Lo que sigue al marcador puede ser:

El contenido real del archivo (código, texto, YAML, etc.)
Una descripción en lenguaje natural de lo que debe contener el archivo


TU TAREA
PASO 0 — ¿Esto es un proyecto o una carcasa?
Antes de extraer archivos, leé el Briefing (si está) y diagnosticá el adjunto.

Es CARCASA si ocurre CUALQUIERA de estas:
- No hay manifiesto de dependencias del stack del briefing (manifest.json de VTEX IO / package.json / pom.xml / build.gradle / requirements.txt / go.mod / *.tf / *.csproj, según corresponda)
- Hay un "binario" que en realidad es un comentario ("no puede ser mostrado como texto plano", placeholder .fig/.docx vacío)
- Los markdowns ya completan entregables de fases posteriores ("se implementó fade-in", lista de áreas ya resuelta)

Si es CARCASA:
- MATERIALIZÁ un proyecto que arranca en el stack del briefing (VTEX IO Store Framework, Angular, Terraform, pytest, Nest, etc.). Incluí manifiesto, punto de entrada y capa de interfaz reales.
- NO copies los markdowns de "solución" como si fueran el producto. Son ruido de generación.
- NO resuelvas las fases del briefing (están marcadas PROHIBIDO). Dejá el hueco pedagógico: el flujo existe, las microinteracciones/calidad/infra que el reto pide NO están hechas.
- Después seguí al PASO 5 (ZIP).

Si es un proyecto REAL (manifiesto + código que compila o arranca):
- Seguí PASO 1 en adelante. 🔴 compilación sí. 🟡 pedagógico no.

PASO 1 — Detección y extracción
Identifica todos los archivos presentes en la cadena. Para cada archivo extrae:

Su ruta completa (ej: src/main/java/com/pragma/Service.java)
Su contenido o descripción

PASO 2 — Clasificación por tipo
Clasifica cada archivo en una de estas categorías:
A) Código fuente (Java, Python, TypeScript, JavaScript, Kotlin, etc.)
B) Configuración / documentación (YAML, properties, Markdown, JSON, txt, etc.)
C) Excel (.xlsx, .xls, .csv)
D) Word (.docx, .doc)
E) Otro tipo de archivo binario o especial
PASO 3 — Clasificación de errores en código fuente

Objetivo prioritario: que el proyecto compile. No corrijas flujo de negocio ni lógica funcional.

Antes de modificar cualquier archivo de código fuente, clasifica cada problema encontrado en una de estas dos categorías:
🔴 ERROR DE COMPILACIÓN — corregir siempre
Son errores que impiden que el proyecto arranque, sin valor pedagógico:

Import faltante o incorrecto
Clase, método o variable referenciada que no existe en ningún archivo del proyecto
Error de sintaxis
Anotación con atributos inválidos
Dependencia ausente en pom.xml, package.json, etc.
Archivo referenciado que no existe y debe ser creado con implementación mínima

→ CORREGIR estos errores.
🟡 PROBLEMA FUNCIONAL O DE CALIDAD — preservar siempre
Son problemas que no impiden compilar. Pueden ser intencionales para el aprendizaje:

Clave secreta hardcodeada ("secret", "password123")
API deprecada que funciona pero tiene reemplazo moderno
Lógica de negocio incorrecta o incompleta
Código redundante o de baja legibilidad
Falta de validaciones en flujo de negocio
Patrones de diseño incorrectos pero funcionales
Concurrencia no segura
Configuración funcional pero no óptima

→ PRESERVAR tal cual. No corregir, no mejorar, no comentar.
PASO 4 — Procesamiento según tipo de archivo
Tipo A — Código fuente
Aplica únicamente las correcciones clasificadas como 🔴 ERROR DE COMPILACIÓN.
No alteres ningún elemento clasificado como 🟡 PROBLEMA FUNCIONAL O DE CALIDAD.
Si falta un archivo referenciado, créalo con la implementación mínima necesaria para compilar.
Tipo B — Configuración / documentación
Extrae el contenido tal cual, sin modificaciones salvo errores evidentes de sintaxis
(ej: YAML mal indentado).
Tipo C — Excel (.xlsx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un archivo Excel funcional con:

Fila de encabezados en negrita con color de fondo distintivo
Columnas con ancho ajustado al contenido
Tipos de dato correctos por columna
Validaciones si la descripción lo indica
Hojas nombradas descriptivamente si hay más de una
Filas de ejemplo si no hay datos reales

Tipo D — Word (.docx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un documento Word funcional con:

Estilos de título (Título 1, Título 2) para jerarquía de secciones
Fuente legible (Calibri o equivalente), tamaño 11-12pt para cuerpo
Márgenes estándar
Tabla de contenido si tiene múltiples secciones
Tablas con encabezados en negrita si aplica

Tipo E — Otro
Genera el archivo con el contenido o estructura más apropiada según la descripción.
PASO 5 — Exportación en ZIP
Empaqueta todos los archivos en un único archivo ZIP descargable respetando exactamente
la estructura de rutas indicada por los marcadores.
El ZIP debe incluir:

Archivos de código con únicamente los errores de compilación corregidos
Archivos de configuración y documentación sin cambios
Archivos nuevos creados para resolver dependencias de compilación faltantes
Archivos Excel y Word generados desde descripción

IMPORTANTE: El ZIP debe estar listo para descargar al finalizar. No preguntes si el usuario
quiere generarlo. Simplemente genera el archivo y proporciona el enlace de descarga; No debes desplegar en el chat el resumen de lo que arreglaste al Zip, solo entregalo.

REGLAS IMPORTANTES

No omitas ningún archivo aunque no tenga errores ni modificaciones
Respeta los nombres y rutas exactas indicadas por los marcadores
Si un archivo no tiene marcador claro, infiere el nombre desde su contenido
Si la cadena contiene solo documentación, placeholders o binarios fake, NO la reproduzcas:
aplicá PASO 0 (materializar el proyecto del briefing). Reproducir la carcasa es un fallo.
No agregues texto después del enlace de descarga del ZIP
No preguntes si el usuario quiere el ZIP: simplemente generalo siempre
Si detectas que falta un archivo de configuración necesario para compilar
(pom.xml, package.json, requirements.txt, build.gradle, etc.), créalo e inclúyelo
inferiendo su contenido desde los imports y frameworks detectados en el código
Nunca corrijas problemas 🟡 aunque parezcan obvios o fáciles de mejorar.
El participante que recibirá este proyecto los debe encontrar y resolver él mismo.


INPUT
Aquí está la cadena con los archivos:

// === ARCHIVO: Cargo.toml ===
[package]
name = "fintech_api"
version = "0.1.0"
edition = "2018"

[dependencies]
actix-web = "4.0"
diesel = { version = "2.0", features = ["postgres"] }
dotenv = "0.15"

[dev-dependencies]
actix-web = "4.0"
diesel = { version = "2.0", features = ["postgres"] }
dotenv = "0.15"

// === ARCHIVO: src/main.rs ===
mod domain;
mod application;
mod infrastructure;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
dotenv().ok();
let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
infrastructure::establish_connection(&database_url);

HttpServer::new(|| {
    App::new()
       .service(web::resource("/transactions").route(web::post().to(infrastructure::create_transaction))
       .route(web::get().to(infrastructure::list_transactions))
       .route(web::put().to(infrastructure::update_transaction))
       .route(web::delete().to(infrastructure::delete_transaction)))
})
.bind("127.0.0.1:8080")?
.run()
.await
}

// === ARCHIVO: src/domain/transaction.rs ===
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub amount: f64,
    pub date: String,
    pub status: String,
}

// === ARCHIVO: src/application/transaction_service.rs ===
use crate::domain::Transaction;
use crate::infrastructure::diesel_repository::DieselRepository;

pub struct TransactionService {
    repository: DieselRepository,
}

impl TransactionService {
    pub fn new(repository: DieselRepository) -> Self {
        TransactionService { repository }
    }

    pub fn create_transaction(&self, transaction: Transaction) -> Result<Transaction, String> {
        // Lógica para crear una transacción
        Ok(transaction)
    }

    pub fn list_transactions(&self) -> Result<Vec<Transaction>, String> {
        // Lógica para listar transacciones
        Ok(vec![])
    }

    pub fn update_transaction(&self, transaction: Transaction) -> Result<Transaction, String> {
        // Lógica para actualizar una transacción
        Ok(transaction)
    }

    pub fn delete_transaction(&self, id: i32) -> Result<(), String> {
        // Lógica para eliminar una transacción
        Ok(())
    }
}

// === ARCHIVO: src/infrastructure/diesel_repository.rs ===
use crate::domain::Transaction;
use diesel::prelude::*;

pub struct DieselRepository;

impl DieselRepository {
    pub fn establish_connection(database_url: &str) {
        let connection = PgConnection::establish(database_url)
           .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    }

    pub fn create_transaction(&self, transaction: Transaction) -> Result<Transaction, String> {
        // Lógica para crear una transacción en la base de datos
        Ok(transaction)
    }

    pub fn list_transactions(&self) -> Result<Vec<Transaction>, String> {
        // Lógica para listar transacciones de la base de datos
        Ok(vec![])
    }

    pub fn update_transaction(&self, transaction: Transaction) -> Result<Transaction, String> {
        // Lógica para actualizar una transacción en la base de datos
        Ok(transaction)
    }

    pub fn delete_transaction(&self, id: i32) -> Result<(), String> {
        // Lógica para eliminar una transacción de la base de datos
        Ok(())
    }
}

// === ARCHIVO: src/infrastructure/actix_handlers.rs ===
use actix_web::{web, HttpResponse, Responder};
use crate::application::transaction_service::TransactionService;
use crate::domain::Transaction;

pub async fn create_transaction(transaction_service: web::Data<TransactionService>, transaction: web::Json<Transaction>) -> impl Responder {
    match transaction_service.create_transaction(transaction.into_inner()) {
        Ok(transaction) => HttpResponse::Ok().json(transaction),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

pub async fn list_transactions(transaction_service: web::Data<TransactionService>) -> impl Responder {
    match transaction_service.list_transactions() {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

pub async fn update_transaction(transaction_service: web::Data<TransactionService>, transaction: web::Json<Transaction>) -> impl Responder {
    match transaction_service.update_transaction(transaction.into_inner()) {
        Ok(transaction) => HttpResponse::Ok().json(transaction),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

pub async fn delete_transaction(transaction_service: web::Data<TransactionService>, id: web::Path<i32>) -> impl Responder {
    match transaction_service.delete_transaction(*id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

// === ARCHIVO: tests/transaction_tests.rs ===
use crate::application::transaction_service::TransactionService;
use crate::domain::Transaction;

#[actix_rt::test]
async fn test_create_transaction() {
    let transaction_service = TransactionService::new(DieselRepository);
    let transaction = Transaction {
        id: 1,
        user_id: 1,
        amount: 100.0,
        date: String::from("2024-07-15"),
        status: String::from("completed"),
    };
    let result = transaction_service.create_transaction(transaction);
    assert!(result.is_ok());
}

// === ARCHIVO: tests/integration_tests.rs ===
use actix_web::test;
use crate::application::transaction_service::TransactionService;
use crate::domain::Transaction;

#[actix_rt::test]
async fn test_integration_create_transaction() {
    let transaction_service = TransactionService::new(DieselRepository);
    let transaction = Transaction {
        id: 1,
        user_id: 1,
        amount: 100.0,
        date: String::from("2024-07-15"),
        status: String::from("completed"),
    };
    let app = test::init_service(App::new()
       .app_data(web::Data::new(transaction_service))
       .service(web::resource("/transactions").route(web::post().to(crate::infrastructure::actix_handlers::create_transaction)))
    ).await;

    let req = test::TestRequest::post()
       .uri("/transactions")
       .set_json(&transaction)
       .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
```
