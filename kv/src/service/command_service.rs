use std::sync::Arc;

use crate::*;

impl CommandService for Hget {
    fn execute(self, store: &Arc<dyn Storage>) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hgetall {
    fn execute(self, store: &Arc<dyn Storage>) -> CommandResponse {
        match store.get_all(&self.table) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hset {
    fn execute(self, store: &Arc<dyn Storage>) -> CommandResponse {
        match self.pair {
            Some(v) => match store.set(&self.table, v.key, v.value.unwrap_or_default()) {
                Ok(Some(v)) => v.into(),
                Ok(None) => Value::default().into(),
                Err(e) => e.into(),
            },
            None => Value::default().into(),
        }
    }
}

impl CommandService for Hdel {
    fn execute(self, store: &Arc<dyn Storage>) -> CommandResponse {
        match store.del(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => Value::default().into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hexist {
    fn execute(self, store: &Arc<dyn Storage>) -> CommandResponse {
        match store.contains(&self.table, &self.key) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hmexist {
    fn execute(self, store: &Arc<dyn Storage>) -> CommandResponse {
        let mut res = true;

        for key in self.keys {
            match store.contains(&self.table, &key) {
                Ok(true) => {}
                Ok(false) => {
                    res = false;
                    break;
                }
                Err(e) => return e.into(),
            }
        }

        res.into()
    }
}

impl CommandService for Hmget {
    fn execute(self, store: &Arc<dyn Storage>) -> CommandResponse {
        let mut res = Vec::new();
        for key in self.keys {
            match store.get(&self.table, &key) {
                Ok(Some(v)) => res.push(v),
                Ok(None) => res.push(Value::default()),
                Err(e) => return e.into(),
            }
        }

        res.into()
    }
}

impl CommandService for Hmset {
    fn execute(self, store: &Arc<dyn Storage>) -> CommandResponse {
        let mut res = Vec::new();
        for kv in self.pairs {
            match store.set(&self.table, kv.key, kv.value.unwrap_or_default()) {
                Ok(Some(v)) => res.push(v),
                Ok(None) => res.push(Value::default()),
                Err(e) => return e.into(),
            }
        }
        res.into()
    }
}

impl CommandService for Hmdel {
    fn execute(self, store: &Arc<dyn Storage>) -> CommandResponse {
        let mut res = Vec::new();

        for key in self.keys {
            match store.del(&self.table, &key) {
                Ok(Some(v)) => res.push(v),
                Ok(None) => res.push(Value::default()),
                Err(e) => return e.into(),
            }
        }

        res.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_request::RequestData;

    #[test]
    fn hset_should_work() {
        let store: Arc<dyn Storage> = Arc::new(MemTable::new());
        let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &[Value::default()], &[]);

        let res = dispatch(cmd, &store);
        assert_res_ok(res, &["world".into()], &[]);
    }

    #[test]
    fn hget_should_work() {
        let store: Arc<dyn Storage> = Arc::new(MemTable::new());
        let cmd = CommandRequest::new_hset("score", "u1", 10.into());
        dispatch(cmd, &store);
        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[10.into()], &[]);
    }

    #[test]
    fn hget_with_non_exist_key_should_return_404() {
        let store: Arc<dyn Storage> = Arc::new(MemTable::new());
        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_error(res, 404, "Not found");
    }

    #[test]
    fn hgetall_should_work() {
        let store: Arc<dyn Storage> = Arc::new(MemTable::new());
        let cmds = vec![
            CommandRequest::new_hset("score", "u1", 10.into()),
            CommandRequest::new_hset("score", "u2", 8.into()),
            CommandRequest::new_hset("score", "u3", 11.into()),
            CommandRequest::new_hset("score", "u1", 6.into()),
        ];
        for cmd in cmds {
            dispatch(cmd, &store);
        }

        let cmd = CommandRequest::new_hgetall("score");
        let res = dispatch(cmd, &store);
        let pairs = &[
            Kvpair::new("u1", 6.into()),
            Kvpair::new("u2", 8.into()),
            Kvpair::new("u3", 11.into()),
        ];
        assert_res_ok(res, &[], pairs);
    }

    #[test]
    fn hdel_should_work() {
        let store: Arc<dyn Storage> = Arc::new(MemTable::new());
        let cmd = CommandRequest::new_hset("score", "u1", 10.into());
        dispatch(cmd, &store);

        let cmd = CommandRequest::new_hdel("score", "u1");
        let res = dispatch(cmd, &store);

        assert_res_ok(res, &[10.into()], &[]);
    }

    #[test]
    fn hmset_should_work() {
        let store: Arc<dyn Storage> = Arc::new(MemTable::new());
        let cmd = CommandRequest::new_hmset(
            "t1",
            vec![
                Kvpair::new("hello1", "world1".into()),
                Kvpair::new("hello2", "world2".into()),
            ],
        );

        println!("{:?}", cmd);

        let res = dispatch(cmd.clone(), &store);

        assert_res_ok(res, &[Value::default(), Value::default()], &[]);
    }

    #[test]
    fn hmget_show_work() {
        let store: Arc<dyn Storage> = Arc::new(MemTable::new());
        let cmd = CommandRequest::new_hmset(
            "score",
            vec![
                Kvpair::new("u1", 10.into()),
                Kvpair::new("u2", 8.into()),
                Kvpair::new("u3", 11.into()),
            ],
        );

        dispatch(cmd, &store);

        let cmd = CommandRequest::new_hmget("score", vec!["u1", "u2", "u3"]);
        let res = dispatch(cmd, &store);

        assert_res_ok(res, &[10.into(), 8.into(), 11.into()], &[]);
    }

    #[test]
    fn hmget_with_non_exist_key_should_return_default() {
        let store: Arc<dyn Storage> = Arc::new(MemTable::new());
        let cmd = CommandRequest::new_hmget("score", vec!["u1", "u2", "u3"]);
        let res = dispatch(cmd, &store);

        assert_res_ok(
            res,
            &[Value::default(), Value::default(), Value::default()],
            &[],
        );
    }

    #[test]
    fn hmdel_should_work() {
        let store: Arc<dyn Storage> = Arc::new(MemTable::new());
        let cmd = CommandRequest::new_hmset(
            "score",
            vec![
                Kvpair::new("u1", 10.into()),
                Kvpair::new("u2", 8.into()),
                Kvpair::new("u3", 11.into()),
            ],
        );

        dispatch(cmd, &store);

        let cmd = CommandRequest::new_hmdel("score", vec!["u1", "u2", "u3"]);
        dispatch(cmd, &store);

        let cdm = CommandRequest::new_hmget("score", vec!["u1", "u2", "u3"]);

        let res = dispatch(cdm, &store);

        assert_res_ok(
            res,
            &[Value::default(), Value::default(), Value::default()],
            &[],
        );
    }

    #[test]
    fn hexist_should_work() {
        let store: Arc<dyn Storage> = Arc::new(MemTable::new());
        let s_cmd = CommandRequest::new_hexist("t1", "hello");
        let res = dispatch(s_cmd.clone(), &store);

        assert_eq!(res.status, 404);

        let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        dispatch(cmd.clone(), &store);

        let res = dispatch(s_cmd, &store);

        assert_eq!(res.status, 200);
    }

    #[test]
    fn hmexist_should_work() {
        let store: Arc<dyn Storage> = Arc::new(MemTable::new());
        let cmd = CommandRequest::new_hmset(
            "score",
            vec![
                Kvpair::new("u1", 10.into()),
                Kvpair::new("u2", 8.into()),
                Kvpair::new("u3", 11.into()),
            ],
        );

        dispatch(cmd, &store);

        let cmd = CommandRequest::new_hmexist("score", vec!["u1", "u2", "u3"]);
        let res = dispatch(cmd, &store);

        assert_eq!(res.status, 200);

        let cmd = CommandRequest::new_hdel("score", "u1");

        dispatch(cmd, &store);

        let cmd = CommandRequest::new_hmexist("score", vec!["u1", "u2", "u3"]);
        let res = dispatch(cmd, &store);

        assert_eq!(res.status, 404);
    }

    // 从 Request 中得到 Response，目前处理 HGET/HGETALL/HSET
    fn dispatch(cmd: CommandRequest, store: &Arc<dyn Storage>) -> CommandResponse {
        match cmd.request_data.unwrap() {
            RequestData::Hget(v) => v.execute(store),
            RequestData::Hgetall(v) => v.execute(store),
            RequestData::Hset(v) => v.execute(store),
            RequestData::Hdel(v) => v.execute(store),
            RequestData::Hexist(v) => v.execute(store),
            RequestData::Hmget(v) => v.execute(store),
            RequestData::Hmset(v) => v.execute(store),
            RequestData::Hmexist(v) => v.execute(store),
            RequestData::Hmdel(v) => v.execute(store),
        }
    }
}
