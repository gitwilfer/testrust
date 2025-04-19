-- migrations/YYYY-MM-DD-HHMMSS_create_attributes_table/down.sql

-- Eliminar el trigger si se creó
-- DROP TRIGGER IF EXISTS update_attributes_updated_at ON attributes;
-- DROP FUNCTION IF EXISTS update_updated_at_column();

-- Eliminar índices
DROP INDEX IF EXISTS idx_attributes_entity_id;

-- Eliminar la tabla
DROP TABLE attributes;