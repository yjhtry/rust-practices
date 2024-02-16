import test from 'ava'

import { hGetAll, hget, hset } from '../index.js'


// Because rust kv lib use memory storage, so we can't test the data persistence
// Todo rust kv lib support persistent storage

test('hget should work!', (t) => {
  let res = hget('t1', 'name')

  t.is(res.status, 404)
})

test('hset should work!', (t) => {
  let res = hset('t1', 'name', 'test')
  t.is(res.status, 200)
})

test('hGetAll should work!', (t) => {
  let res = hGetAll('t1')

  console.log(res)

  t.is(res.status, 200)
})
