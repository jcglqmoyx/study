create table user
(
    id        integer
        constraint user_pk
            primary key autoincrement,
    username  text,
    join_time integer,
    is_admin  numeric,
    wechat_id text,
    is_active integer
);

create index user_username_index
    on user (username);

create index user_wechat_id_index
    on user (wechat_id);

