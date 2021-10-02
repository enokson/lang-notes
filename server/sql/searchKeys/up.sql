create table if not exists searchKeys (
    id int auto_increment,
    definitionId int not null,
    searchKey varchar(256) not null,
    primary key (id)
)