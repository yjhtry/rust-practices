import { query } from './index'

function test_query() {
  let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

  // 使用 sql 从 URL 里获取数据
  let sql = `SELECT location name, total_cases, new_cases, total_deaths, new_deaths 
    FROM ${url} where new_deaths >= 200 ORDER BY new_cases DESC`

  console.log(query(sql, 'json'))
}

test_query()
