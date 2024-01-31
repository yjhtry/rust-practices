const {example_sql, query} = require( './index.node')

function test_example_sql() {
  console.log(example_sql())
}

function test_query() {
  let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

  // 使用 sql 从 URL 里获取数据
  let sql = `SELECT location name, total_cases, new_cases, total_deaths, new_deaths 
    FROM ${url} where new_deaths >= 200 ORDER BY new_cases DESC`

  console.log(query(sql, 'csv'))
}

const test_case = process.argv[2];

if (test_case === 'example_sql') {
  test_example_sql()
} else if (test_case === 'query') {
  test_query()
} else {
  console.log('test_case is not defined, please use "example_sql" or "query"')
}
