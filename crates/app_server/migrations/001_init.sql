create table if not exists technicians
(
    id         integer primary key generated always as identity,
    name       text        not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
    );

create type service_type as enum ('Maintenance', 'Installation', 'Repair', 'Emergency');

create table if not exists services
(
    id               integer primary key generated always as identity,
    service_type     service_type not null,
    duration_minutes smallint     NOT NULL CHECK (duration_minutes > 0),
    price_cents      integer      not null check ( price_cents > 0 ),
    created_at       timestamptz  not null default now(),
    updated_at       timestamptz  not null default now()
    );

create table customers
(
    id           integer primary key generated always as identity,
    name         text        not null,
    email        text        not null,
    constraint email_valid check (email ~* '^[A-Za-z0-9._%+\-]+@[A-Za-z0-9.\-]+\.[A-Za-z]{2,}$'),
    phone_number text        not null check (phone_number ~ '^[0-9]{10}$'::text),
    created_at   timestamptz not null default now(),
    updated_at   timestamptz not null default now()
);

create table appointments
(
    id             integer primary key generated always as identity,
    scheduled_time timestamptz,
    technician_id  integer     not null references technicians (id),
    service_id     integer     not null references services (id),
    customer_id    integer     not null references customers (id),
    created_at     timestamptz not null default now(),
    updated_at     timestamptz not null default now()
);

create unique index customers_email_lower_idx on customers (lower(email));

