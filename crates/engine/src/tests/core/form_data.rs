// @see: https://github.com/web-platform-tests/wpt/tree/e45a9f9e65d039c76817ee2a6a1ef02c9311a1cb/xhr/formdata
#[cfg(test)]
mod tests {
    use anyhow::{Ok, Result};
    use regex::Regex;
    use serde_json::json;

    use crate::tests::test_utils::context::Context;

    // @see: https://github.com/web-platform-tests/wpt/blob/e45a9f9e65d039c76817ee2a6a1ef02c9311a1cb/xhr/formdata/append.any.js
    #[test]
    fn test_form_data_append() -> Result<()> {
        let mut ctx = Context::new();

        // Append
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.append('key', 'value1');
            var formDataValue = formData.get('key');
        "#,
        )?;

        assert_eq!(
            "value1",
            ctx.global.get_property("formDataValue")?.as_str()?
        );

        // Multiple Appends to the same key
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.append('key', 'value1');
            formData.append('key', 'value2');
            var formDataValue = formData.get('key');
        "#,
        )?;

        assert_eq!(
            "value1",
            ctx.global.get_property("formDataValue")?.as_str()?
        );

        // Append Null
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.append('key', undefined);
            var formDataValue = formData.get('key');
        "#,
        )?;

        assert_eq!(
            "undefined",
            ctx.global.get_property("formDataValue")?.as_str()?
        );

        // Multiple Appends to the same key first as undefined
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.append('key', undefined);
            formData.append('key', 'value2');
            var formDataValue = formData.get('key');
        "#,
        )?;

        assert_eq!(
            "undefined",
            ctx.global.get_property("formDataValue")?.as_str()?
        );

        // Append Null
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.append('key', null);
            var formDataValue = formData.get('key');
        "#,
        )?;

        assert_eq!("null", ctx.global.get_property("formDataValue")?.as_str()?);

        // Multiple Appends to the same key first as null
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.append('key', null);
            formData.append('key', 'value2');
            var formDataValue = formData.get('key');
        "#,
        )?;

        assert_eq!("null", ctx.global.get_property("formDataValue")?.as_str()?);

        // Append Blob
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.append('key', new Blob());
            var formDataValue = JSON.stringify(formData.get('key'));
        "#,
        )?;

        assert_eq!(
            json!({"size":0,"type":""}).to_string(),
            ctx.global.get_property("formDataValue")?.as_str()?
        );

        // Multiple Appends none String or Blob and convert to String
        ctx.eval(
            r#"
            var formData = new FormData();
            
            formData.append('key', 1);
            formData.append('key', true);
            formData.append('key', []);
            formData.append('key', {});

            var formDataValues = JSON.stringify(formData.getAll('key'));
        "#,
        )?;

        assert_eq!(
            r#"["1","true","","[object Object]"]"#,
            ctx.global.get_property("formDataValues")?.as_str()?
        );

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/e45a9f9e65d039c76817ee2a6a1ef02c9311a1cb/xhr/formdata/constructor.any.js
    #[test]
    fn test_form_data_constructor_errors() -> Result<()> {
        let mut ctx = Context::new();

        let re_sequence = Regex::new(
            r"^Uncaught TypeError: Failed to construct 'FormData': parameters are not allowed",
        )
        .unwrap();

        assert!(re_sequence.is_match(
            &ctx.eval(r#"var formData = new FormData(null);"#,)
                .unwrap_err()
                .to_string()
        ));

        assert!(re_sequence.is_match(
            &ctx.eval(r#"var formData = new FormData("string");"#,)
                .unwrap_err()
                .to_string()
        ));

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/e45a9f9e65d039c76817ee2a6a1ef02c9311a1cb/xhr/formdata/delete.any.js
    #[test]
    fn test_form_data_delete() -> Result<()> {
        let mut ctx = Context::new();

        // Delete
        ctx.eval(
            r#"
                var formData = new FormData();
                formData.append('key', 'value1');
                formData.delete('key');
                var formDataValue = formData.get('key');
            "#,
        )?;

        assert_eq!("null", ctx.global.get_property("formDataValue")?.as_str()?);

        // Delete non existing key
        ctx.eval(
            r#"
                var formData = new FormData();
                formData.append('key1', 'value1');
                formData.append('key2', 'value2');
                formData.delete('nil');
                var formDataValue = formData.get('key1');
            "#,
        )?;

        assert_eq!(
            "value1",
            ctx.global.get_property("formDataValue")?.as_str()?
        );

        // Delete other key
        ctx.eval(
            r#"
                var formData = new FormData();
                formData.append('key_1', 'value1');
                formData.append('key_2', 'value2');
                formData.delete('key_1');
                var formDataValue1 = formData.get('key_1');
                var formDataValue2 = formData.get('key_2');
            "#,
        )?;

        assert_eq!("null", ctx.global.get_property("formDataValue1")?.as_str()?);

        assert_eq!(
            "value2",
            ctx.global.get_property("formDataValue2")?.as_str()?
        );

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/e45a9f9e65d039c76817ee2a6a1ef02c9311a1cb/xhr/formdata/get.any.js
    #[test]
    fn test_form_data_get_and_get_all() -> Result<()> {
        let mut ctx = Context::new();

        // Get
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.append('key', 'value1');
            formData.append('key', 'value2');
            var formDataValue = formData.get('key');
        "#,
        )?;

        assert_eq!(
            "value1",
            ctx.global.get_property("formDataValue")?.as_str()?
        );

        // Get Null 1
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.append('key', 'value1');
            formData.append('key', 'value2');
            var formDataValue = formData.get('nil');
        "#,
        )?;

        assert_eq!("null", ctx.global.get_property("formDataValue")?.as_str()?);

        // Get Null 2
        ctx.eval(
            r#"
            var formData = new FormData();
            var formDataValue = formData.get('key');
        "#,
        )?;

        assert_eq!("null", ctx.global.get_property("formDataValue")?.as_str()?);

        // Get All
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.append('key', 'value1');
            formData.append('key', 'value2');
            var formDataValue = formData.getAll('key');
        "#,
        )?;

        assert_eq!(
            "value1,value2",
            ctx.global.get_property("formDataValue")?.as_str()?
        );

        // Get All Empty 1
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.append('key', 'value1');
            formData.append('key', 'value2');
            var formDataValue = formData.getAll('nill');
        "#,
        )?;

        assert_eq!("", ctx.global.get_property("formDataValue")?.as_str()?);

        // Get All Empty 2
        ctx.eval(
            r#"
            var formData = new FormData();
            var formDataValue = formData.getAll('key');
        "#,
        )?;

        assert_eq!("", ctx.global.get_property("formDataValue")?.as_str()?);

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/e45a9f9e65d039c76817ee2a6a1ef02c9311a1cb/xhr/formdata/has.any.js
    #[test]
    fn test_form_data_has() -> Result<()> {
        let mut ctx = Context::new();

        // Has
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.append('key', 'value1');
            formData.append('key', 'value2');
            var formDataValue = formData.has('key');
        "#,
        )?;

        assert_eq!("true", ctx.global.get_property("formDataValue")?.as_str()?);

        // Has Empty 1
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.append('key', 'value1');
            formData.append('key', 'value2');
            var formDataValue = formData.has('nil');
        "#,
        )?;

        assert_eq!("false", ctx.global.get_property("formDataValue")?.as_str()?);

        // Has Empty 2
        ctx.eval(
            r#"
            var formData = new FormData();
            var formDataValue = formData.has('key');
        "#,
        )?;

        assert_eq!("false", ctx.global.get_property("formDataValue")?.as_str()?);

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/e45a9f9e65d039c76817ee2a6a1ef02c9311a1cb/xhr/formdata/set-blob.any.js
    #[test]
    fn test_form_data_set_blob() -> Result<()> {
        let mut ctx = Context::new();

        // NOTE: The File class is not implemented

        // Set Blob
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.set('key', new Blob());
            var formDataValue = formData.get('key');
            var blob = JSON.stringify({
                size: formDataValue.size,
                type: formDataValue.type
            });
        "#,
        )?;

        assert_eq!(
            json!({"size":0,"type":""}).to_string(),
            ctx.global.get_property("blob")?.as_str()?
        );

        // Set Blob with type
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.set('key', new Blob([], { type: "text/plain" }));
            var formDataValue = formData.get('key');
            var blob = JSON.stringify({
                size: formDataValue.size,
                type: formDataValue.type
            });
        "#,
        )?;

        assert_eq!(
            json!({"size":0,"type":"text/plain"}).to_string(),
            ctx.global.get_property("blob")?.as_str()?
        );

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/e45a9f9e65d039c76817ee2a6a1ef02c9311a1cb/xhr/formdata/set.any.js
    #[test]
    fn test_form_data_set() -> Result<()> {
        let mut ctx = Context::new();

        // Set 1
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.set('key', 'value1');
            var formDataValue = formData.get('key');
        "#,
        )?;

        assert_eq!(
            "value1",
            ctx.global.get_property("formDataValue")?.as_str()?
        );

        // Set 2
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.set('key', 'value1');
            formData.set('key', 'value2');
            var formDataValue = formData.getAll('key');
        "#,
        )?;

        assert_eq!(
            "value2",
            ctx.global.get_property("formDataValue")?.as_str()?
        );

        // Set Undefined 1
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.set('key', undefined);
            var formDataValue = formData.get('key');
        "#,
        )?;

        assert_eq!(
            "undefined",
            ctx.global.get_property("formDataValue")?.as_str()?
        );

        // Set Undefined 2
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.set('key', undefined);
            formData.set('key', 'value1');
            var formDataValue = formData.get('key');
        "#,
        )?;

        assert_eq!(
            "value1",
            ctx.global.get_property("formDataValue")?.as_str()?
        );

        // Set Null 1
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.set('key', null);
            var formDataValue = formData.get('key');
        "#,
        )?;

        assert_eq!("null", ctx.global.get_property("formDataValue")?.as_str()?);

        // Set Null 2
        ctx.eval(
            r#"
            var formData = new FormData();
            formData.set('key', null);
            formData.set('key', 'value1');
            var formDataValue = formData.get('key');
        "#,
        )?;

        assert_eq!(
            "value1",
            ctx.global.get_property("formDataValue")?.as_str()?
        );

        Ok(())
    }
}
