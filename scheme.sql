create extension if not exists "uuid-ossp";
drop table public.docs;
create table public.docs
(
    id          uuid primary key not null default uuid_generate_v4(),
    doc         jsonb            not null default '{}',
    flag_active bool             not null default true
);
drop table public.edges;
create table public.edges
(
    id          uuid primary key not null default uuid_generate_v4(),
    tail_id     uuid             not null,
    head_id     uuid             not null,
    label       text             not null,
    properties  jsonb            not null default '{}',
    flag_active bool             not null default true
);