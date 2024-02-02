use crate::entity::user::User;

pub fn concatenate_usernames(users: &Vec<User>) -> String {
    users
        .iter()
        .map(|user| {
            let mut s = user.id.clone().unwrap().to_string() + ". " + &user.username.clone().unwrap();
            if user.is_active.unwrap() == 0 {
                s.push_str("(未激活)")
            }
            s
        }
        )
        .collect::<Vec<String>>()
        .join("\n")
}