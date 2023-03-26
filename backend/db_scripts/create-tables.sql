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

-- bridge table for terms and topics
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
*/
CREATE TABLE platform.questions (
	id serial NOT NULL,
	question text NOT NULL,
	topic_id int NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (topic_id) REFERENCES platform.topics(id), 
	CONSTRAINT unique_question UNIQUE(question)
);

