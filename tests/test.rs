use std::sync::Arc;

use sqlx_pro::core::sql::delete::Del;
use sqlx_pro::core::sql::insert::Insert;
use sqlx_pro_macro::table_name;

#[table_name("测试表名")]
struct UserT {
    name: String,
    age: i32,
    phone: String,
}

#[test]
fn test() {
    let s = UserT {
        name: "张三".to_string(),
        age: 18,
        phone: "18888888888".to_string(),
    };
    let string = Insert::default(&s)
        // .table(arc.clone())
        // .set_fields(arc.clone())
        .set_values(vec!["李四".to_string(), "20".to_string(), "123".to_string()])
        .build();
    // println!("{}", vec.join(","));
    println!("select语句：{}", string);


    let del = Del::default(&s)
        .eq("name","1")
        .eq("age","3")
        .build();
    println!("delete语句：{}",del)

}
