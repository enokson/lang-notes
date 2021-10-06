insert into translations (lang_id, definition)
values ($1, $2) returning id;