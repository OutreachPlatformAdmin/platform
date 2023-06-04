# Endpoint Definitions and Usage

## Singular Record Endpoints 

### `/topic`

### Parameters

#### Example Usage 

#### Example output

### `/term`

### Parameters

#### Example Usage 

#### Example output

### `/source` 

### Parameters

#### Example Usage 
`source?id=1`

Returns a single source record.

#### Example output

````
{
    "id": 1,
    "name": "dictionary austerity",
    "url": "https://www.merriam-webster.com/dictionary/austerity",
    "author": null,
    "author_url": null,
    "media_type": "Web",
    "image_url": null,
    "image_type": null,
    "ai_generated": false
}
````

## Multiple Record Endpoints

### `/topics`

Returns all available topics.

#### Example Usage 

#### Example output

### `/terms`

Returns all available terms.

#### Example Usage 

#### Example output

### `/sources` 

Returns all available sources.

#### Example Usage 

#### Example output

## Relational Endpoints

### `/terms-from-topic`

Returns all of the terms related to a given topic.

### Parameters

`topic`: string

#### Example Usage 

`/terms-from-topic?topic=neoliberalism`

#### Example output

### Entity Creation Endpoints

### `/new-topic`

### Parameters

#### Example Usage 

#### Example output

### `/new-term`

### Parameters

#### Example Usage 

#### Example output

### `/new-source` 

### Parameters

#### Example Usage 

#### Example output
