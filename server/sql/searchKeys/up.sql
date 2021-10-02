create table if not exists searchKeys (
    id int,
    definitionId int not null,
    key varchar(256) not null,
    primary key (id)
)