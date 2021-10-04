const post = {
    method: 'post',
    url: '/definition',
    schema: {
      body: {
        name: { type: 'string' },
        excitement: { type: 'integer' }
      },
      response: {
        200: {
          type: 'object',
          properties: {
            hello: { type: 'string' }
          }
        }
      }
    },
    handler: function (request, reply) {
      reply.send({ hello: 'world' })
    }
  }