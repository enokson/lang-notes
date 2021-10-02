
const translation = {
    type: 'object',
    properties: {
        lang: { type: 'string' },
        value: { type: 'string' }
    }
}

const definition = {
    type: 'object',
    properties: {
        value: { type: 'string' },
        translations: { type: 'array', items: translation },
        group: { type: 'string' },
        pronounciation: { type: 'string' },
        example: { type: 'string' },
        search: { type: 'array', items: { type: 'string' } }
    }
}

const definitionCluster = {
    type: 'object',
    properties: {
        definitions: { type: 'array', items: definition }
    }    
}

module.exports = { entry }