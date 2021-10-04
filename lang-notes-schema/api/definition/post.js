const requestBody = {
    type: 'object',
    properties: {
        wordGroup: {
            type: 'object',
            properties: {
                id: { type: 'number' },
                name: { type: 'string' }
            },
            required: []
        },
        clusterId: { type: [ 'number', 'null' ] },
        definition: {
            type: 'object',
            properties: {
                pronounciation: { type: 'string' },
                word: { type: 'string' },
                prefixes: { type: 'string' },
                suffixes: { type: 'string' },
                definition: { type: 'string' }
            },
            required: [ 'word', 'definition' ]
        },
        examples: {
            type: 'array',
            items: {

            }
        },
        translations: {
            type: 'array',
            items: {
                
            }
        }
    },
    required: [ 'wordGroup', 'clusterId', 'definition' ]
}