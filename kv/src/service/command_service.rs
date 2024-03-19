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

    mod memory {
        use super::*;

        #[test]
        fn memory_hset_should_work() {
            let store: Arc<dyn Storage> = Arc::new(MemTable::new());
            test_hset(store);
        }

        #[test]
        fn memory_hget_should_work() {
            let store: Arc<dyn Storage> = Arc::new(MemTable::new());
            test_hget(store);
        }

        #[test]
        fn memory_hget_with_non_exist_key_should_return_404() {
            let store: Arc<dyn Storage> = Arc::new(MemTable::new());
            test_hget_with_404(store);
        }

        #[test]
        fn memory_hgetall_should_work() {
            let store: Arc<dyn Storage> = Arc::new(MemTable::new());
            test_hgetall(store);
        }

        #[test]
        fn memory_hdel_should_work() {
            let store: Arc<dyn Storage> = Arc::new(MemTable::new());
            test_hdel(store);
        }

        #[test]
        fn memory_hmset_should_work() {
            let store: Arc<dyn Storage> = Arc::new(MemTable::new());
            test_hmset(store);
        }

        #[test]
        fn memory_hmget_show_work() {
            let store: Arc<dyn Storage> = Arc::new(MemTable::new());
            test_hmget(store);
        }

        #[test]
        fn memory_hmget_with_non_exist_key_should_return_default() {
            let store: Arc<dyn Storage> = Arc::new(MemTable::new());
            test_hmget_with_non_exist_key(store);
        }

        #[test]
        fn memory_hmdel_should_work() {
            let store: Arc<dyn Storage> = Arc::new(MemTable::new());
            test_hmdel(store);
        }

        #[test]
        fn memory_hexist_should_work() {
            let store: Arc<dyn Storage> = Arc::new(MemTable::new());
            test_hexist(store);
        }

        #[test]
        fn memory_hmexist_should_work() {
            let store: Arc<dyn Storage> = Arc::new(MemTable::new());
            test_hmexist(store);
        }
    }

    mod sled {
        use super::*;

        #[test]
        fn sled_hset_should_work() {
            let store: Arc<dyn Storage> = Arc::new(get_sled_store());
            test_hset(store);
        }

        #[test]
        fn sled_hget_should_work() {
            let store: Arc<dyn Storage> = Arc::new(get_sled_store());
            test_hget(store);
        }

        #[test]
        fn sled_hget_with_non_exist_key_should_return_404() {
            let store: Arc<dyn Storage> = Arc::new(get_sled_store());
            test_hget_with_404(store);
        }

        #[test]
        fn sled_hgetall_should_work() {
            let store: Arc<dyn Storage> = Arc::new(get_sled_store());
            test_hgetall(store);
        }

        #[test]
        fn sled_hdel_should_work() {
            let store: Arc<dyn Storage> = Arc::new(get_sled_store());
            test_hdel(store);
        }

        #[test]
        fn sled_hmset_should_work() {
            let store: Arc<dyn Storage> = Arc::new(get_sled_store());
            test_hmset(store);
        }

        #[test]
        fn sled_hmget_show_work() {
            let store: Arc<dyn Storage> = Arc::new(get_sled_store());
            test_hmget(store);
        }

        #[test]
        fn sled_hmget_with_non_exist_key_should_return_default() {
            let store: Arc<dyn Storage> = Arc::new(get_sled_store());
            test_hmget_with_non_exist_key(store);
        }

        #[test]
        fn sled_hmdel_should_work() {
            let store: Arc<dyn Storage> = Arc::new(get_sled_store());
            test_hmdel(store);
        }

        #[test]
        fn sled_hexist_should_work() {
            let store: Arc<dyn Storage> = Arc::new(get_sled_store());
            test_hexist(store);
        }

        #[test]
        fn sled_hmexist_should_work() {
            let store: Arc<dyn Storage> = Arc::new(get_sled_store());
            test_hmexist(store);
        }
    }

    fn test_hset(store: Arc<dyn Storage>) {
        let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &[Value::default()], &[]);

        let res = dispatch(cmd, &store);
        assert_res_ok(res, &["world".into()], &[]);
    }

    fn test_hget(store: Arc<dyn Storage>) {
        let cmd = CommandRequest::new_hset("score", "u1", 10.into());
        dispatch(cmd, &store);
        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[10.into()], &[]);
    }

    fn test_hget_with_404(store: Arc<dyn Storage>) {
        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_error(res, 404, "Not found");
    }

    fn test_hgetall(store: Arc<dyn Storage>) {
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

    fn test_hdel(store: Arc<dyn Storage>) {
        let cmd = CommandRequest::new_hset("score", "u1", 10.into());
        dispatch(cmd, &store);

        let cmd = CommandRequest::new_hdel("score", "u1");
        let res = dispatch(cmd, &store);

        assert_res_ok(res, &[10.into()], &[]);
    }

    fn test_hmset(store: Arc<dyn Storage>) {
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

    fn test_hmget(store: Arc<dyn Storage>) {
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

    fn test_hmget_with_non_exist_key(store: Arc<dyn Storage>) {
        let cmd = CommandRequest::new_hmget("score", vec!["u1", "u2", "u3"]);
        let res = dispatch(cmd, &store);

        assert_res_ok(
            res,
            &[Value::default(), Value::default(), Value::default()],
            &[],
        );
    }

    fn test_hmdel(store: Arc<dyn Storage>) {
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

    fn test_hexist(store: Arc<dyn Storage>) {
        let s_cmd = CommandRequest::new_hexist("t1", "hello");
        let res = dispatch(s_cmd.clone(), &store);

        assert_eq!(res.status, 404);

        let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        dispatch(cmd.clone(), &store);

        let res = dispatch(s_cmd, &store);

        assert_eq!(res.status, 200);
    }

    fn test_hmexist(store: Arc<dyn Storage>) {
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
