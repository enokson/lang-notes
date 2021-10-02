const wordGroupsRow = {
    type: 'object',
    properties: {
        id: { type: 'number' },
        name: { type: 'string' }
    }
}

const languagesRow = {
    type: 'object',
    properties: {
        id: { type: 'number' },
        name: { type: 'string' }
    }
}

const translationsRow = {
    type: 'object',
    properties: {
        id: { type: 'number' },
        langId: { type: 'number', "$comment": 'this prop correlates to the id of the languages table' },
        definition: { type: 'string' },
        // literal: { type: 'string' },
        misc: { type: 'string' }
    }
}

const searchKeysRow = {
    type: 'object',
    properties: {
        id: { type: 'number' },
        definitionId: { type: 'number' },
        searchKey: { type: 'string' }
    }
}

const examplesRow = {
    type: 'object',
    properties: {
        id: { type: 'number' },
        definitionId: { type: 'number' },
        translationId: { type: 'number' },
        example: { type: 'string' }
    },
    required: [ "id", "example" ]
}

const definitionsRow = {
    type: 'object',
    properties: {
        id: { type: 'number' },
        wordGroupId: { type: 'number' },
        clusterId: { type: 'number' },
        pronounciation: { type: 'string' },
        word: { type: 'string' },
        prefixes: { type: 'string' },
        suffixes: { type: 'string' },
        definition: { type: 'string' }
    }
}

const clusterRow = {
    type: 'object',
    properties: {
        id: { type: 'number' }
    }
}

const translationSchema = {
    type: 'object',
    properties: {
        translation: translationsRow,
        lang: languagesRow,
        examples: { type: 'array', items: examplesRow }
    }
}

const definitionSchema = {
    type: 'object',
    properties: {
        definition: definitionsRow,
        wordGroup: wordGroupsRow,
        translations: { type: 'array', items: translationSchema },
        searchKeys: { type: 'array', items: searchKeysRow },
        examples: { type: 'array', items: examplesRow }
    }
}

const clusterSchema = { 
    type: 'object',
    properties: {
        cluster: clusterRow,
        definitions: { type: 'array', items: definitionSchema }
    }
}

const clustersSchema = { type: 'array', items: clusterSchema }
