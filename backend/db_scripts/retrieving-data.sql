select * from platform.topics;
/*
sample output:
"id"	"topic"
1	"new topic"
2	"neoliberalism"
*/

select * from platform.terms;
/*
"id"	"term"
1	"new term"
2	"new term 2"
3	"new term 3"
*/


select * from platform.terms_to_topics;
/*
"term_id"	"topic_id"
1	1
2	1
3	2
*/

-- get all terms that corresopnd to 'new topic' (term_id: 1)
select * from platform.terms_to_topics where term_id = 1;
/*
"term_id"	"topic_id"
1	1
2	1
*/


select * from platform.questions;
/*
"id"	"question"	"topic_id"
1	"question about new topic"	1
*/