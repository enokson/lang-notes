create table if not exists examples (
    id int auto_increment primary key,
    parent_type int not null,
    parent_id int,
    example varchar(256) not null
);