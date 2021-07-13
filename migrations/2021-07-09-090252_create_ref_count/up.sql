CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE EXTENSION IF NOT EXISTS btree_gin;

CREATE TABLE IF NOT EXISTS
karyon_entity (
    id              BIGINT NOT NULL,
    avatar_entity   BIGINT,
    owner_entity    BIGINT NOT NULL,
    editor_entity   BIGINT NOT NULL,
    viewer_entity   BIGINT NOT NULL,
    author_entity   BIGINT NOT NULL,
    create_time     TIMESTAMP WITH TIME ZONE NOT NULL,
    modify_time     TIMESTAMP WITH TIME ZONE NOT NULL,

    PRIMARY KEY(id)
);

CREATE INDEX IF NOT EXISTS
karyon_entity_idx_permission ON karyon_entity USING BTREE (
    owner_entity,
    editor_entity,
    viewer_entity
);

CREATE TABLE IF NOT EXISTS
karyon_attr (
    id              BIGINT NOT NULL,
    owner_entity    BIGINT NOT NULL,
    editor_entity   BIGINT NOT NULL,
    viewer_entity   BIGINT NOT NULL,
    author_entity   BIGINT NOT NULL,
    create_time     TIMESTAMP WITH TIME ZONE NOT NULL,
    modify_time     TIMESTAMP WITH TIME ZONE NOT NULL,

    PRIMARY KEY(id)
);

CREATE INDEX IF NOT EXISTS
karyon_attr_idx_permission ON karyon_attr USING BTREE (
    owner_entity,
    editor_entity,
    viewer_entity
);

CREATE TABLE IF NOT EXISTS
karyon_i18n (
    id              BIGINT NOT NULL,
    entity          BIGINT NOT NULL,
    attr            BIGINT NOT NULL,
    lang            VARCHAR NOT NULL,
    value           VARCHAR NOT NULL,

    UNIQUE(entity, attr, lang),
    PRIMARY KEY(id)
);

CREATE INDEX IF NOT EXISTS
karyon_i18n_idx_entity_attr ON karyon_i18n USING BTREE (
    entity          ASC,
    attr            ASC NULLS LAST
);

CREATE INDEX IF NOT EXISTS
karyon_i18n_idx_value_attr ON karyon_i18n USING GIN (
    value           GIN_TRGM_OPS,
    attr            INT8_OPS
);

CREATE TABLE IF NOT EXISTS karyon_link (
    id              BIGINT NOT NULL,
    attr            BIGINT,
    src_entity      BIGINT NOT NULL,
    dest_entity     BIGINT NOT NULL,
    direct          BOOLEAN NOT NULL,
    ref_count       BIGINT NOT NULL,

    PRIMARY KEY(id)
);

CREATE INDEX IF NOT EXISTS
karyon_link_idx_src_count ON karyon_link USING BTREE (
    src_entity      ASC,
    ref_count       ASC
);

CREATE INDEX IF NOT EXISTS
karyon_link_idx_dest_count ON karyon_link USING BTREE (
    dest_entity     ASC,
    ref_count       ASC
);

CREATE INDEX IF NOT EXISTS
karyon_link_idx_attr ON karyon_link USING BTREE (
    attr            ASC NULLS LAST
);
