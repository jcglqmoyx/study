pub mod query {
    use rbatis::executor::RBatisTxExecutor;
    use rbatis::RBatis;

    use crate::entity::user::User;

    pub async fn find_by_wechat_id_via_tx(tx: &RBatisTxExecutor, wechat_id: &String) -> Option<User> {
        match User::select_by_column(tx, "wechat_id", wechat_id).await {
            Ok(users) => {
                match users.len() {
                    0 => None,
                    _ => Some(users[0].clone())
                }
            }
            Err(_) => None
        }
    }

    pub async fn find_by_wechat_id_via_rb(rb: &RBatis, wechat_id: &String) -> Option<User> {
        match User::select_by_column(rb, "wechat_id", wechat_id).await {
            Ok(users) => {
                match users.len() {
                    0 => None,
                    _ => Some(users[0].clone())
                }
            }
            Err(_) => None
        }
    }

    pub async fn find_by_username(tx: &RBatisTxExecutor, username: String) -> Option<User> {
        match User::select_by_column(tx, "username", username).await {
            Ok(users) => {
                match users.len() {
                    0 => None,
                    _ => Some(users[0].clone())
                }
            }
            Err(_) => None
        }
    }

    pub async fn is_admin(wechat_id: String, rb: &RBatis) -> bool {
        match User::select_by_column(rb, "wechat_id", wechat_id).await {
            Ok(users) => {
                if !users.is_empty() && users[0].is_admin.unwrap_or(0) == 1 {
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    pub async fn count_user(tx: &RBatisTxExecutor) -> u64 {
        tx
            .query_decode("select count(1) as count from user", vec![])
            .await
            .unwrap_or(0)
    }

    pub async fn list_user(rb: &RBatis) -> Vec<User> {
        User::select_all(rb).await.unwrap()
    }
}