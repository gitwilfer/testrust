# Configuración de Bases de Datos

## Arquitectura Dual de Acceso a Datos

La aplicación utiliza dos sistemas ORM:

1. **Diesel** (ORM principal):
   - Maneja todas las operaciones transaccionales (CREATE, UPDATE, DELETE)
   - Requiere esquema de base de datos definido en src/Infrastructure/Persistence/schema.rs
   - Configuración en .env (MAIN_DATABASE_URL)

2. **SQLx** (para consultas):
   - Usado exclusivamente para operaciones de lectura no bloqueantes  
   - Requiere directorio migrations/ con estructura de la base de datos
   - Misma configuración que Diesel (comparte conexión)

## Requisitos de Migraciones SQLx

El directorio `migrations/` debe contener:

- Archivos .sql con los cambios de esquema
- Deben coincidir con el esquema actual de la base de datos
- Formato: VERSION__DESCRIPTION.sql (ej: 0001__initial_tables.sql)

## Solución de Problemas

Si hay errores de migración faltantes:

1. Verificar el esquema actual con:
```bash
diesel print-schema
```

2. Crear migraciones SQLx que reflejen ese esquema

3. Ejecutar con:
```bash
cargo run
