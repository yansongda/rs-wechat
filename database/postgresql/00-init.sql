-- auto-generated definition
create table yansongda.user
(
    id         bigserial
        constraint pk_user_id
            primary key,
    open_id    varchar(128)                           not null,
    avatar     text,
    nickname   varchar(128)             default '微信用户'::character varying,
    slogan     varchar(246),
    created_at timestamp with time zone default now() not null,
    updated_at timestamp with time zone default now() not null
);

alter table yansongda.user
    owner to miniprogram;

create unique index uk_user_open_id
    on yansongda.user (open_id);

-- auto-generated definition
create table yansongda.totp
(
    id         bigserial
        constraint pk_totp_id
            primary key,
    user_id    bigint                                 not null,
    username   varchar(128)                           not null,
    issuer     varchar(128),
    secret     varchar(256)                           not null,
    config      jsonb                                          ,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp with time zone default now() not null
);

alter table yansongda.totp
    owner to miniprogram;

create index idx_totp_user
    on yansongda.totp (user_id);

-- auto-generated definition
create table yansongda.short_url
(
    id         bigserial
        constraint pk_short_url_id
            primary key,
    user_id    bigint                                 not null,
    short      varchar(64)                            not null
        constraint uk_short_url_short
            unique,
    link       text,
    visit      integer                  default 0     not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp with time zone default now() not null
);

alter table yansongda.short_url
    owner to miniprogram;

create index idx_short_url_user
    on yansongda.short_url (user_id);


