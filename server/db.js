const { promises } = require('fs')
// const { join } from 'path'
const { readFile } = promises



const createTables = async conn => {
    const readThenCreate = async sqlPath => {
        let sql = await readFile(sqlPath, 'utf-8')
        await conn.query(sql)
    }
    await Promise.all([
        readThenCreate('./sql/clusters/up.sql'),
        readThenCreate('./sql/definitions/up.sql'),
        readThenCreate('./sql/examples/up.sql'),
        readThenCreate('./sql/languages/up.sql'),
        readThenCreate('./sql/searchKeys/up.sql'),
        readThenCreate('./sql/translations/up.sql'),
        readThenCreate('./sql/wordGroups/up.sql')
    ])
}

module.exports = { createTables }