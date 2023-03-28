CREATE TABLE platform.topics (
	id serial NOT NULL,
	topic text NOT NULL, 
	PRIMARY KEY (id),
	CONSTRAINT unique_topic UNIQUE(topic)
);

CREATE TABLE platform.terms (
	id serial NOT NULL,
	term text NOT NULL,
	PRIMARY KEY (id),
	CONSTRAINT unique_term UNIQUE(term)
);

/*
bridge table for terms and topics
query patterns: 
- get all of the terms for a given topic
- get all of the topics for a given term
*/
CREATE TABLE platform.terms_to_topics (
	term_id int NOT NULL,
	topic_id int NOT NULL,
	FOREIGN KEY (term_id) REFERENCES platform.terms(id),
	FOREIGN KEY (topic_id) REFERENCES platform.topics(id),
	UNIQUE(term_id, topic_id)
);

/*
I defined each question as only corresponding to a single topic.
If we want to change that, we can set up another bridge table for 
questions to topics.

query pattern: get all of the questions related to a given topic.
*/
CREATE TABLE platform.questions (
	id serial NOT NULL,
	question text NOT NULL,
	topic_id int NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (topic_id) REFERENCES platform.topics(id), 
	CONSTRAINT unique_question UNIQUE(question)
);

/*
this can be used when building the mind-map
*/
CREATE TABLE platform.related_topics (
	id serial NOT NULL,
	parent_id int NOT NULL,
	child_id int NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (parent_id) REFERENCES platform.topics(id),
	FOREIGN KEY (child_id) REFERENCES platform.topics(id),
	UNIQUE(parent_id, child_id)
);

CREATE TABLE platform.articles (
	id serial NOT NULL,
	title text,
	author text,
	publish_date DATE,
	PRIMARY KEY (id)
);

CREATE TABLE platform.article_to_topics (
	article_id int NOT NULL,
	topic_id int NOT NULL,
	FOREIGN KEY (article_id) REFERENCES platform.articles(id),
	FOREIGN KEY (topic_id) REFERENCES platform.topics(id),
	UNIQUE(article_id, topic_id)
);

CREATE TABLE platform.article_to_terms (
	article_id int NOT NULL,
	term_id int NOT NULL,
	FOREIGN KEY (article_id) REFERENCES platform.articles(id),
	FOREIGN KEY (term_id) REFERENCES platform.terms(id),
	UNIQUE(article_id, term_id)
);


CREATE TABLE platform.article_to_questions (
	article_id int NOT NULL,
	question_id int NOT NULL,
	FOREIGN KEY (article_id) REFERENCES platform.articles(id),
	FOREIGN KEY (question_id) REFERENCES platform.questions(id),
	UNIQUE(article_id, question_id)
);