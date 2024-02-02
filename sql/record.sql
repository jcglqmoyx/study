create table record
(
    id      integer
        constraint record_pk
            primary key,
    user_id integer,
    time    integer,
    url     text
);

create index record_time_index
    on record (time);