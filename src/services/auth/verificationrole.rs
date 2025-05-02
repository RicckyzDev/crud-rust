pub async fn user_has_role(
    db: &sqlx::PgPool,
    user_id: &i32,
    role: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM user_roles ur
            JOIN roles r ON ur.role_id = r.id
            WHERE ur.user_id = $1 AND r.name = $2
        )
        "#,
        user_id,
        role
    )
    .fetch_one(db)
    .await?;

    Ok(result.unwrap_or(false))
}
