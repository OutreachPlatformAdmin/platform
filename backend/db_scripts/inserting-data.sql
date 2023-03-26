-- we don't need to specify the `id` column bc it is serial 
-- so it will auto-increment
INSERT INTO platform.topics (topic) VALUES ('new topic');


INSERT INTO platform.terms (term) VALUES ('new term');
INSERT INTO platform.terms (term) VALUES ('new term 2');
INSERT INTO platform.terms (term) VALUES ('new term 3');

/*
For now we manually need to update the bridge table.
We can later set up a trigger to do this, or ensure the function that updates
these tables in the code makes the required updates.
*/
INSERT INTO platform.terms_to_topics (term_id, topic_id) VALUES (1, 1),
(2, 1);
INSERT INTO platform.terms_to_topics (term_id, topic_id) VALUES (3, 2);



INSERT INTO platform.questions (question, topic_id) VALUES ('question about new topic', 1);
