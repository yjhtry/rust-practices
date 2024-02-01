import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import './App.css'
import { Button, Input } from 'antd'

const url = 'https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv'
const testSql = `SELECT location name, total_cases, new_cases, total_deaths, new_deaths 
FROM ${url} where new_deaths >= 200 ORDER BY new_cases DESC`

function App() {
  const [loading, setLoading] = useState(false)
  const [sql, setSql] = useState(testSql)
  const [data, setData] = useState('')

  const onInput: React.ChangeEventHandler<HTMLTextAreaElement> = (e) => {
    setSql(e.target.value)
  }

  const onQuery = async () => {
    setLoading(true)
    try {
      const res: string = await invoke('query', { sql, output: 'csv' })
      setData(res)
    }
    finally {
      setLoading(false)
    }
  }

  return (
    <div className="p-10">
      <Input.TextArea autoSize value={sql} onChange={onInput} />
      <div className="flex justify-end mt-6">
        <Button disabled={loading} loading={loading} type="primary" onClick={onQuery}>查询</Button>
      </div>
      <pre>
        {JSON.stringify(data, null, 2)}
      </pre>
    </div>
  )
}

export default App
