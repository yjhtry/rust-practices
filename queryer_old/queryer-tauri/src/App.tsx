import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Button, Input } from "antd";

const url = 'https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv'
const testSql = `SELECT location name, total_cases, new_cases, total_deaths, new_deaths 
FROM ${url} where new_deaths >= 200 ORDER BY new_cases DESC`

function App() {
  const [sql, setSql] = useState(testSql);
  const [data, setData] = useState("");


  const onInput: React.ChangeEventHandler<HTMLTextAreaElement> = (e) => {
    setSql(e.target.value);
  }

  const onQuery = async () => {
    const res: string = await invoke("query", { sql, output: "csv" });
    setData(res);
  }

  return (
    <div className="px-10">
      <Input.TextArea autoSize value={sql}  onChange={onInput} />
      <Button type="primary" onClick={onQuery}>查询</Button>
      <pre>
        {data}
      </pre>
    </div>
  );
}

export default App;
