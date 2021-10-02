const { promises } = require('fs')
// const { join } from 'path'
const { readFile } = promises



const createTables = async conn => {
    const readThenCreate = async sqlPath => {
        let sql = await readFile(sqlPath, 'utf-8')
        await conn.query(sql)
    }
    // const createDefinitionsSql = await readFile('./sql/definition/up.sql', 'utf-8')
    // await conn.query(createDefinitionsSql)
    await Promise.all([
        readThenCreate('./sql/definitions/up.sql'),
        readThenCreate('./sql/translations/up.sql'),
        readThenCreate('./sql/clusters/up.sql')
    ])
}

module.exports = { createTables }