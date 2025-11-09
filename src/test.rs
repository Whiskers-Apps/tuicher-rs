#[test]
fn test_query() {
    use crate::utils::Query;

    let phrase = "add youtube music https://youtube.com";
    let query = Query::new(phrase).unwrap();

    assert_eq!(query.keyword.clone().unwrap(), "add");

    assert_eq!(
        query.get_query().clone(),
        "youtube music https://youtube.com"
    );

    assert_eq!(query.full_text.clone(), phrase);
}
