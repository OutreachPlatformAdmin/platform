# Endpoint Definitions and Usage

## Singular Record Endpoints 

### `/topic`

Returns a single topic record.

### Parameters

`id`: int, topic id.

#### Example Usage 

`/topic?id=1`

### `/term`

Returns a single term record.

### Parameters

`id`: int, term id.

#### Example Usage 

`/term?id=1`

### `/source`

Returns a single source record. 

### Parameters

`id`: int, source id.

#### Example Usage 
`source?id=1`


## Multiple Record Endpoints

### `/topics`

Returns all available topics.

### `/terms`

Returns all available terms.

### `/sources` 

Returns all available sources.

## Relational Endpoints

### `/terms-from-topic`

Returns all of the terms related to a given topic.

### Parameters

`topic`: string

#### Example Usage 

`/terms-from-topic?topic=neoliberalism`

## Entity Creation Endpoints

### `/new-topic`

#### POST Body Parameters

`name`: string  
`is_verified`: bool  
`brief_description`: string, optional  
`full_description`: string, optional  
`bullet_points`: string[], optional  
`examples`: string[], optional  
`parallels`: string[], optional  
`ai_brief_description`: string, optional  
`ai_full_description`: string, optional  
`ai_bullet_points`: string[], optional  
`ai_parallels`: string[], optional  
`ai_examples`: string[], optional  
`related_terms`: string[], optional  
`related_topics`: string[], optional  
`related_sources`: string[], optional  

#### Example Usage 

See `/new-term` example usage.

### POST `/new-term`

#### POST Body Parameters

`name`: string  
`is_verified`: bool  
`brief_description`: string, optional  
`full_description`: string, optional  
`bullet_points`: string[], optional  
`examples`: string[], optional  
`parallels`: string[], optional  
`ai_brief_description`: string, optional  
`ai_full_description`: string, optional  
`ai_bullet_points`: string[], optional  
`ai_parallels`: string[], optional  
`ai_examples`: string[], optional  
`related_terms`: string[], optional  
`related_topics`: string[], optional  
`related_sources`: string[], optional  

#### Example Usage 

```
POST localhost:3000/new-term
BODY:
{ 
    "name":"deregulation",
    "is_verified": true,
    "brief_description": "the process of removing or reducing government regulations and restrictions on industries, businesses, and markets.",
    "bullet_points" : ["bullet1", "bullet2"],
    "examples": ["example1", "example2"]
}
```

### `/new-source` 

### Parameters

#### Example Usage 



### POST `/link-entities`

Link two entities that already exist in the database.

### POST Body Parameters

`parent_entity_type`: string, must be an existing entity type, e.g. `topic`
`child_entity_type`: string, must be an existing entity type, e.g. `term`
`parent_id`: int, the id of the parent entity 
`related_topic_ids`: int[], optional, list of IDs the parent entity is related to
`related_term_ids`: int[], optional, list of IDs the parent entity is related to
`related_source_ids`: int[], optional, list of IDS the parent entity is related to

#### Example Usage 

```
POST localhost:3000/link-entities
BODY:
{
    "parent_entity_type": "term",
    "child_entity_type": "topic",
    "parent_id": 3,
    "related_topic_ids": [1]
}
```


