import test from "ava";

import { hGetAll, hget, hset } from "../index.js";


test("hset should work!", (t) => {
  let res = hset("set_t1", "name", "test");

  t.deepEqual(res, {
    message: "",
    pairs: [],
    status: 200,
    values: [{ value: null }],
  });
});

test("hget should work!", (t) => {
  let res = hget("get_t1", "name");

  t.deepEqual(res, {
    message: "Not found for table: get_t1, key: name",
    pairs: [],
    status: 404,
    values: [],
  });

  hset("get_t1", "name", "test");

  res = hget("get_t1", "name");

  t.deepEqual(res, {
    message: "",
    pairs: [],
    status: 200,
    values: [{ value: { String: "test" } }],
  });
});

test("hGetAll should work!", (t) => {
  let res = hGetAll("get_all_t1");

  t.deepEqual(res, { message: "", pairs: [], status: 200, values: [] });

  hset("get_all_t1", "name", "test");
  hset("get_all_t1", "age", 18);

  res = hGetAll("get_all_t1");

  console.log(JSON.stringify(res, null, 2));

  t.deepEqual(res, {
    message: "",
    pairs: [
      { key: "name", value: { value: { String: "test" } } },
      { key: "age", value: { value: { Integer: 18 } } },
    ],
    status: 200,
    values: [],
  });
});
