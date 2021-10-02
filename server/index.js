// Require the framework and instantiate it
const fastify = require('fastify')({ logger: true })
const mysql = require('mysql2/promise'); 
const db = require('./db')


// const { MongoClient } = require("mongodb");
// Connection URI
// const url = 'mongodb://localhost:27017';
// const client = new MongoClient(url);
// const db = client.db('lang')
// const words = db.collection('words')

const Ajv = require("ajv")
const ajv = new Ajv()

// const schema = require('./schema')

// fastify.decorate('db', db)
// fastify.decorate('words', words)

// Declare a route
fastify.get('/', async (request, reply) => {
  return { hello: 'world' }
})

// fastify.route({
//     method: 'POST',
//     url: '/word',
//     schema: {
//         body: schema.entry
//     },
//     handler: async (request, reply) => {
//         await reply.words.insertOne(request.body)
//         return { ok: true }
//     }
// })

// fastify.route({
//     method: 'PUT',
//     url: '/word',
//     schema: {
//         body: schema.entry
//     },
//     handler: async (request, reply) => {
//         await reply.words.updateOne(request.body)
//         return { ok: true }
//     }
// })

// fastify.route({
//     method: 'GET',
//     url: '/word',
//     schema: {
//         querystring: { 
//             word: { type: 'string' }
//         }
//     },
//     handler: async (request, reply) => {
//         const docs = await reply.words.findOne({ name: request.querystring.word })
//         return { ok: true, docs }
//     }
// })

// fastify.route({
//     method: 'DELETE',
//     url: '/word',
//     schema: {
//         querystring: { 
//             word: { type: 'string' }
//         }
//     },
//     handler: async (request, reply) => {
//         const docs = await reply.words.findOne({ name: request.querystring.word })
//         return { ok: true, docs }
//     }
// })

// fastify.route({
//     method: 'GET',
//     url: '/search/word',
//     schema: {
//         querystring: { 
//             word: { type: 'string' }
//         }
//     },
//     handler: async (request, reply) => {
//         const docs = await reply.words.find({ word: { $text: request.querystring.word } })
//         return {
//             ok: true,
//             docs
//         }
//     }
// })


// Run the server!
const start = async () => {
  try {

    const conn = await mysql.createConnection({
        host:'localhost', 
        user: 'root',
        password: 'example',
        database: 'lang'
    });

    await db.createTables(conn)

    fastify.decorate('conn', conn)
    await fastify.listen(3300)
  } catch (err) {
    fastify.log.error(err)
    process.exit(1)
  }
}
start()