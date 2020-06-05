use async_graphql::*;

#[async_std::test]
pub async fn test_custom_directive() {
    #[Enum(internal)]
    enum Role {
        Guest,
        Member,
        Admin,
    }

    struct Auth {
        role: Role,
    }

    #[Directive]
    impl Auth {
        async fn before_field_resolve(
            &self,
            ctx: &ContextDirective<'_>,
            role: Role,
        ) -> FieldResult<()> {
            if let Some(role) = ctx.data_opt::<Role>() {
                if *role == self.role {
                    return Ok(());
                }
            }
            Err("forbidden".into())
        }
    }
}