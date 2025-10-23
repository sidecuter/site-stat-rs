use crate::auth::create_token;

#[rstest::fixture]
#[once]
pub fn jwt_token() -> String {
    create_token(1, "524c9b6806b8f7ae95c56747d35432c7").unwrap()
}
