-- migrations/YYYY-MM-DD-HHMMSS_create_attributes_table/up.sql

CREATE TABLE attributes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_id UUID NOT NULL REFERENCES logical_entities(id) ON DELETE CASCADE,
    data_type_id UUID NOT NULL REFERENCES data_types(id) ON DELETE RESTRICT, -- Asume que data_types existe
    name TEXT NOT NULL,
    description TEXT,
    is_required BOOLEAN NOT NULL DEFAULT FALSE,
    position SMALLINT NOT NULL DEFAULT 0,
    is_unique SMALLINT, -- Tipo confirmado por el usuario
    default_value TEXT,
    validation_regex TEXT,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_by UUID REFERENCES users(id) ON DELETE SET NULL,
    updated_at TIMESTAMPTZ, -- Se actualiza manualmente o con trigger
    status SMALLINT NOT NULL DEFAULT 1,

    -- Restricción para asegurar que el nombre del atributo sea único dentro de una entidad
    CONSTRAINT unique_entity_attribute_name UNIQUE (entity_id, name)
);

-- Índice opcional para búsquedas comunes
CREATE INDEX idx_attributes_entity_id ON attributes(entity_id);

-- Trigger opcional para actualizar updated_at automáticamente
-- CREATE OR REPLACE FUNCTION update_updated_at_column()
-- RETURNS TRIGGER AS $$
-- BEGIN
--    NEW.updated_at = now();
--    RETURN NEW;
-- END;
-- $$ language 'plpgsql';
--
-- CREATE TRIGGER update_attributes_updated_at
-- BEFORE UPDATE ON attributes
-- FOR EACH ROW
-- EXECUTE FUNCTION update_updated_at_column();