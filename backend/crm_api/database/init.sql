CREATE DATABASE platform;
/*
connecting to the new db in psql: 
\connect platform
*/
\c platform;
CREATE SCHEMA platform;

CREATE TYPE media_type AS ENUM ('wiki_article', 'book', 'dictionary', 'website', 'print');
CREATE TYPE image_type AS ENUM ('pdf', 'png', 'tiff', 'jpeg', 'gif');

/*
Table terms {
  id int [pk, increment]
  
  // store if 100% of content is human created
  isVerified boolean

  // data fields
  term varchar

  briefDescription varchar

  fullDescription varchar
  
  bulletPoints [varchar]

  examples [varchar]

  parallels [varchar]

  resources [int] [ref: > sources.id]

  // AI-generated data fields 
  AIbriefDescription varchar

  AIfullDescription varchar
  
  AIbulletPoints [varchar]

  AIexamples [varchar]

  AIparallels [varchar]

  AIresources [int] [ref: > sources.id]

}
*/

CREATE TABLE platform.sources (
	id serial NOT NULL,
	name text,
	url text,
	media_type media_type, -- ENUM defined above
	image_url text, 
	image_type image_type, -- ENUM defined above
	PRIMARY KEY (id)
);

CREATE TABLE platform.topics (
	id serial NOT NULL,
	topic text NOT NULL,
	definition text NOT NULL,
	ai_definition text,
	source_id int,
	FOREIGN KEY (source_id) REFERENCES platform.sources(id), 
	PRIMARY KEY (id),
	CONSTRAINT unique_topic UNIQUE(topic)
);

CREATE TABLE platform.terms (
	id serial NOT NULL,
	term text NOT NULL,
	definition text NOT NULL,
	ai_definition text,
	source_id int,
	is_verified int,
	FOREIGN KEY (source_id) REFERENCES platform.sources(id),
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

CREATE TABLE platform.articles_to_topics (
	article_id int NOT NULL,
	topic_id int NOT NULL,
	FOREIGN KEY (article_id) REFERENCES platform.articles(id),
	FOREIGN KEY (topic_id) REFERENCES platform.topics(id),
	UNIQUE(article_id, topic_id)
);

CREATE TABLE platform.articles_to_terms (
	article_id int NOT NULL,
	term_id int NOT NULL,
	FOREIGN KEY (article_id) REFERENCES platform.articles(id),
	FOREIGN KEY (term_id) REFERENCES platform.terms(id),
	UNIQUE(article_id, term_id)
);


CREATE TABLE platform.articles_to_questions (
	article_id int NOT NULL,
	question_id int NOT NULL,
	FOREIGN KEY (article_id) REFERENCES platform.articles(id),
	FOREIGN KEY (question_id) REFERENCES platform.questions(id),
	UNIQUE(article_id, question_id)
);

----------------- Insertion of Sample Data -----------------

-- we don't need to specify the `id` column bc it is serial 
-- so it will auto-increment
INSERT INTO platform.sources (url, media_type) VALUES ('https://www.merriam-webster.com/dictionary/austerity', 'dictionary');
INSERT INTO platform.sources (url, media_type) VALUES ('https://en.wikipedia.org/wiki/Neoliberalism', 'wiki_article');
INSERT INTO platform.sources (url, media_type) VALUES ('https://en.wikipedia.org/wiki/Capitalism', 'wiki_article');

INSERT INTO platform.topics (topic, definition, source_id) VALUES ('capitalism', ' an economic system based on the private ownership of the means of production and their operation for profit.', 3);

INSERT INTO platform.terms (term, definition, source_id) VALUES ('austerity', 'difficult economic conditions created by government measures to reduce a budget deficit, especially by reducing public expenditure', 1);
INSERT INTO platform.terms (term, definition, source_id) VALUES ('neoliberalism', 'a term used to signify the late-20th century political reappearance of 19th-century ideas associated with free-market capitalism after it fell into decline following the Second World War', 2);


/*
For now we manually need to update the bridge table.
We can later set up a trigger to do this, or ensure the function that updates
these tables in the code makes the required updates.
*/
INSERT INTO platform.terms_to_topics (term_id, topic_id) VALUES (1, 1),
(2, 1);


INSERT INTO platform.questions (question, topic_id) VALUES ('What is capitalism?', 1);

-- INSERT INTO platform.related_topics (parent_id, child_id) VALUES (1, 2);

INSERT INTO platform.articles (title, author, publish_date) VALUES ('title1', 'author1', '2023-03-27');

INSERT INTO platform.articles_to_topics VALUES (1, 1);
INSERT INTO platform.articles_to_terms VALUES (1,1), (1,2);
INSERT INTO platform.articles_to_questions VALUES (1,1);

