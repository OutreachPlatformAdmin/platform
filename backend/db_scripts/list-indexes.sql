/*
List indexes in the platform schema
*/
SELECT
    tablename,
    indexname,
    indexdef
FROM
    pg_indexes
WHERE
    schemaname = 'platform'
ORDER BY
    tablename,
    indexname;

/*
Example output showing that indexes get created when PRIMARY KEY 
and UNIQUE() constraints are specified in the table definition. 

"topics"	"topics_pkey"	"CREATE UNIQUE INDEX topics_pkey ON platform.topics USING btree (id)"
"topics"	"unique_topic"	"CREATE UNIQUE INDEX unique_topic ON platform.topics USING btree (topic)"
*/
