insert into examples (parent_type, parent_id, examples)
values ($1, $2, $3) returning id;