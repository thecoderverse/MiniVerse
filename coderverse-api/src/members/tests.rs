#[cfg(test)]
pub mod tests {
    use crate::database::create_pool;
    use crate::members::models::{Member, NewMember};
    use crate::members::services::create_member;

    #[tokio::test]
    async fn test_create_member() {
        let pool = create_pool().await;
        let random_name = rand::random::<u64>().to_string();
        let random_email = rand::random::<u64>().to_string();

        let new_member = NewMember {
            name: random_name.clone(),
            email: random_email.clone(),
        };

        let member = create_member(new_member, &pool).await.unwrap();

        assert_eq!(member.name, random_name);
        assert_eq!(member.email, random_email);
    }

    #[tokio::test]
    async fn test_create_member_invalid() {
        let pool = create_pool().await;
        let new_member = NewMember {
            name: "".to_string(),
            email: "".to_string(),
        };

        let member = create_member(new_member, &pool).await;

        assert!(member.is_err());
    }

    #[tokio::test]
    async fn test_get_members() {
        let pool = create_pool().await;
        let members = crate::members::services::get_members(&pool).await.unwrap();

        assert!(members.len() > 0);
    }

    #[tokio::test]
    async fn test_get_member() {
        let pool = create_pool().await;
        let members = crate::members::services::get_members(&pool).await.unwrap();
        let member = crate::members::services::get_member(members[0].id as u64, &pool).await.unwrap();

        assert_eq!(member.id, members[0].id);
    }

    #[tokio::test]
    async fn test_update_member_name() {
        let pool = create_pool().await;
        let members = crate::members::services::get_members(&pool).await.unwrap();
        let member = crate::members::services::get_member(members[0].id as u64, &pool).await.unwrap();

        let updated_member = Member {
            id: member.id,
            name: "Updated Name".to_string(),
            email: member.email.clone(),
            photo_url: member.photo_url.clone(),
        };

        let member = crate::members::services::update_member(member.id as u64, updated_member, &pool).await.unwrap();

        assert_eq!(member.name, "Updated Name");
    }

    #[tokio::test]
    async fn test_update_member_email() {
        let pool = create_pool().await;
        let members = crate::members::services::get_members(&pool).await.unwrap();
        let member = crate::members::services::get_member(members[0].id as u64, &pool).await.unwrap();

        let updated_member = Member {
            id: member.id,
            name: member.name.clone(),
            email: "qwe@qwe.com".to_string(),
            photo_url: member.photo_url.clone(),
        };

        let member = crate::members::services::update_member(member.id as u64, updated_member, &pool).await.unwrap();

        assert_eq!(member.email, "qwe@qwe.com");
    }
}