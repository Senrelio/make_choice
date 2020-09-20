-- noinspection SpellCheckingInspection @ extension/"uuid-ossp"

create extension if not exists "uuid-ossp";

create table requirements
(
    id  uuid primary key default uuid_generate_v4(),
    doc json not null
);

create table edges
(
    id          uuid primary key default uuid_generate_v4(),
    tail_vertex uuid not null references requirements_target (id),
    head_vertex uuid not null references requirements_target (id),
    label       text not null,
    properties  json
);