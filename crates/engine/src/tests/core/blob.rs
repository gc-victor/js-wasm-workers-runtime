// @see: https://developer.mozilla.org/en-US/docs/Web/API/Blob
// @see: https://fetch.spec.whatwg.org/#response-class
// @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/
// @see: https://github.com/node-fetch/fetch-blob/blob/57e4daef36081936581d14509b6cc618d87ab9e2/test/test-wpt-in-node.js
#[cfg(test)]
mod tests {
    use anyhow::{Ok, Result};
    use regex::Regex;
    use serde_json::json;

    use crate::tests::test_utils::context::Context;

    // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-constructor.any.js
    #[test]
    fn test_response_constructor() -> Result<()> {
        let mut ctx = Context::new();

        // Blob constructor with no arguments
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-constructor.any.js#L13
        ctx.eval(
            r#"
            var blob = new Blob();
            var blob_instance = blob instanceof Blob;
            var blob_stringify = JSON.stringify(blob);
            "#,
        )?;

        assert_eq!(
            json!({"size":0,"type":""}).to_string(),
            ctx.global.get_property("blob_stringify")?.as_str()?
        );
        assert_eq!("true", ctx.global.get_property("blob_instance")?.as_str()?);

        // Blob constructor with undefined as first argument
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-constructor.any.js#L28
        ctx.eval(
            r#"
            var blob = new Blob(undefined);
            var blob_stringify = JSON.stringify(blob);
            "#,
        )?;

        assert_eq!(
            json!({"size":0,"type":""}).to_string(),
            ctx.global.get_property("blob_stringify")?.as_str()?
        );

        // A plain object with @@iterator should be treated as a sequence for the blobParts argument
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-constructor.any.js#L58
        ctx.eval(
            r#"
            var blob = new Blob({
                [Symbol.iterator]: Array.prototype[Symbol.iterator],
            });
            var blob_stringify = JSON.stringify(blob);
            "#,
        )?;

        assert_eq!(
            json!({"size":0,"type":""}).to_string(),
            ctx.global.get_property("blob_stringify")?.as_str()?
        );

        // A plain object with custom @@iterator should be treated as a sequence for the blobParts argument
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-constructor.any.js#L67
        ctx.eval(
            r#"
            var blob = new Blob({
                [Symbol.iterator]() {
                    var i = 0;
                    return {
                        next: () => [
                            {done:false, value:'ab'},
                            {done:false, value:'cde'},
                            {done:true}
                        ][i++]
                    };
                }
            });
            var blob_stringify = JSON.stringify(blob);
            "#,
        )?;

        assert_eq!(
            json!({"size":5,"type":""}).to_string(),
            ctx.global.get_property("blob_stringify")?.as_str()?
        );

        // A plain object with @@iterator and a length property should be treated as a sequence for the blobParts argument
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-constructor.any.js#L81
        ctx.eval(
            r#"
            async function handler() {
                const blob = new Blob({
                    [Symbol.iterator]: Array.prototype[Symbol.iterator],
                        0: "PASS",
                        length: 1
                });
                const blob_text = await blob.text();

                return {blob: { size: blob.size, type: blob.type }, blob_text};
            }
            "#,
        )?;

        assert_eq!(
            json!({"blob":{"size":4,"type":""}, "blob_text":"PASS"}).to_string(),
            ctx.get_handler_value()?
        );

        // A String object should be treated as a sequence for the blobParts argument.
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-constructor.any.js#L92
        ctx.eval(
            r#"
            async function handler() {
                const blob = new Blob(new String("xyz"));
                const blob_text = await blob.text();

                return {blob: { size: blob.size, type: blob.type }, blob_text};
            }
            "#,
        )?;

        assert_eq!(
            json!({"blob":{"size":3,"type":""}, "blob_text":"xyz"}).to_string(),
            ctx.get_handler_value()?
        );

        // A Uint8Array object should be treated as a sequence for the blobParts argument
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-constructor.any.js#L99
        ctx.eval(
            r#"
            async function handler() {
                const blob = new Blob(new Uint8Array([1, 2, 3]));
                const blob_text = await blob.text();

                return {blob: { size: blob.size, type: blob.type }, blob_text};
            }
            "#,
        )?;

        assert_eq!(
            json!({"blob":{"size":3,"type":""}, "blob_text":"123"}).to_string(),
            ctx.get_handler_value()?
        );

        // ToString should be called on elements of the blobParts array.
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-constructor.any.js#L251
        ctx.eval(
            r#"
            async function handler() {
                const blob = new Blob([
                    null,
                    undefined,
                    true,
                    false,
                    0,
                    1,
                    new String("stringobject"),
                    [],
                    ['x', 'y'],
                    {},
                    { 0: "FAIL", length: 1 },
                    { toString: function() { return "stringA"; } },
                    { toString: undefined, valueOf: function() { return "stringB"; } },
                    { valueOf: function() { assert_unreached("Should not call valueOf if toString is present on the prototype."); } }
                  ]);
                const blob_text = await blob.text();

                return {blob_text};
            }
            "#,
        )?;

        assert_eq!(
            json!({"blob_text":"nullundefinedtruefalse01stringobjectx,y[object Object][object Object]stringAstringB[object Object]"}).to_string(),
            ctx.get_handler_value()?
        );

        Ok(())
    }

    // Passing non-objects, Dates and RegExps for blobParts should throw a TypeError
    // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-constructor.any.js#L37
    #[test]
    fn test_response_constructor_errors() -> Result<()> {
        let mut ctx = Context::new();

        let re_sequence = Regex::new(
            r"^Uncaught TypeError: Failed to construct 'Blob': The provided value cannot be converted to a sequence",
        )
        .unwrap();

        assert!(re_sequence.is_match(
            &ctx.eval(r#"var blob = new Blob(null);"#,)
                .unwrap_err()
                .to_string()
        ));

        assert!(re_sequence.is_match(
            &ctx.eval(r#"var blob = new Blob(true);"#,)
                .unwrap_err()
                .to_string()
        ));

        assert!(re_sequence.is_match(
            &ctx.eval(r#"var blob = new Blob(false);"#,)
                .unwrap_err()
                .to_string()
        ));

        assert!(re_sequence.is_match(
            &ctx.eval(r#"var blob = new Blob(0);"#,)
                .unwrap_err()
                .to_string()
        ));

        assert!(re_sequence.is_match(
            &ctx.eval(r#"var blob = new Blob(1);"#,)
                .unwrap_err()
                .to_string()
        ));

        assert!(re_sequence.is_match(
            &ctx.eval(r#"var blob = new Blob(1.5);"#,)
                .unwrap_err()
                .to_string()
        ));

        assert!(re_sequence.is_match(
            &ctx.eval(r#"var blob = new Blob("FAIL");"#,)
                .unwrap_err()
                .to_string()
        ));

        let re_iterator = Regex::new(
            r"^Uncaught TypeError: Failed to construct 'Blob': The object must have a callable @@iterator property",
        )
        .unwrap();

        assert!(re_iterator.is_match(
            &ctx.eval(r#"var blob = new Blob(new Date());"#,)
                .unwrap_err()
                .to_string()
        ));

        assert!(re_iterator.is_match(
            &ctx.eval(r#"var blob = new Blob(new RegExp());"#,)
                .unwrap_err()
                .to_string()
        ));

        assert!(re_iterator.is_match(
            &ctx.eval(r#"var blob = new Blob({});"#,)
                .unwrap_err()
                .to_string()
        ));

        assert!(re_iterator.is_match(
            &ctx.eval(r#"var blob = new Blob({ 0: "FAIL", length: 1 });"#,)
                .unwrap_err()
                .to_string()
        ));

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-array-buffer.any.js
    #[test]
    fn test_response_array_buffer() -> Result<()> {
        let mut ctx = Context::new();

        // With data
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-array-buffer.any.js#L5
        ctx.eval(
            r#"
            async function handler() {
                const input_arr = new TextEncoder().encode("PASS");
                const blob = new Blob([input_arr]);
                const array_buffer = await blob.arrayBuffer();
                const is_array_buffer_instance = array_buffer instanceof ArrayBuffer;
                const uint8array_array_buffer_and_input_arr_are_equal = JSON.stringify(new Uint8Array(array_buffer)) === JSON.stringify(input_arr);

                return {is_array_buffer_instance, uint8array_array_buffer_and_input_arr_are_equal};
            }
            "#,
        )?;

        assert_eq!(
            json!({"is_array_buffer_instance":true, "uint8array_array_buffer_and_input_arr_are_equal": true})
                .to_string(),
            ctx.get_handler_value()?
        );

        // With empty Blob data
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-array-buffer.any.js#L13
        ctx.eval(
            r#"
            async function handler() {
                const input_arr = new TextEncoder().encode("");
                const blob = new Blob([input_arr]);
                const array_buffer = await blob.arrayBuffer();
                const is_array_buffer_instance = array_buffer instanceof ArrayBuffer;
                const uint8array_array_buffer_and_input_arr_are_equal = JSON.stringify(new Uint8Array(array_buffer)) === JSON.stringify(input_arr);

                return {is_array_buffer_instance, uint8array_array_buffer_and_input_arr_are_equal};
            }
            "#,
        )?;

        assert_eq!(
            json!({"is_array_buffer_instance":true, "uint8array_array_buffer_and_input_arr_are_equal": true})
                .to_string(),
            ctx.get_handler_value()?
        );

        // With non-ascii input
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-array-buffer.any.js#L21
        ctx.eval(
            r#"
            async function handler() {
                const input_arr = new TextEncoder().encode("\u08B8\u000a");
                const blob = new Blob([input_arr]);
                const array_buffer = await blob.arrayBuffer();
                const is_array_buffer_instance = array_buffer instanceof ArrayBuffer;
                const uint8array_array_buffer_and_input_arr_are_equal = JSON.stringify(new Uint8Array(array_buffer)) === JSON.stringify(input_arr);

                return {is_array_buffer_instance, uint8array_array_buffer_and_input_arr_are_equal};
            }
            "#,
        )?;

        assert_eq!(
            json!({"is_array_buffer_instance":true, "uint8array_array_buffer_and_input_arr_are_equal": true})
                .to_string(),
            ctx.get_handler_value()?
        );

        // With non-unicode input
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-array-buffer.any.js#L28
        ctx.eval(
            r#"
            async function handler() {
                const input_arr = [8, 241, 48, 123, 151];
                const typed_arr = new Uint8Array(input_arr);
                const blob = new Blob([typed_arr]);
                const array_buffer = await blob.arrayBuffer();
                const is_array_buffer_instance = array_buffer instanceof ArrayBuffer;
                const uint8array_array_buffer_and_input_arr_are_equal = JSON.stringify(new Uint8Array(array_buffer)) === JSON.stringify(typed_arr);

                return {is_array_buffer_instance, uint8array_array_buffer_and_input_arr_are_equal};
            }
            "#,
        )?;

        assert_eq!(
            json!({"is_array_buffer_instance":true, "uint8array_array_buffer_and_input_arr_are_equal": true})
                .to_string(),
            ctx.get_handler_value()?
        );

        // Concurrent reads
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-array-buffer.any.js#L36
        ctx.eval(
            r#"
            async function handler() {
                const input_arr = new TextEncoder().encode("PASS");
                const blob = new Blob([input_arr]);
                const array_buffer_results = await Promise.all([
                    blob.arrayBuffer(),
                    blob.arrayBuffer(),
                    blob.arrayBuffer()
                ]);

                let is_array_buffer_instance = [];
                let uint8array_array_buffer_and_input_arr_are_equal = [];

                for (let array_buffer of array_buffer_results) {
                     is_array_buffer_instance.push(array_buffer instanceof ArrayBuffer);
                     uint8array_array_buffer_and_input_arr_are_equal.push(JSON.stringify(new Uint8Array(array_buffer)) === JSON.stringify(input_arr));
                }

                return {is_array_buffer_instance, uint8array_array_buffer_and_input_arr_are_equal};

                
            }
            "#,
        )?;

        assert_eq!(
            json!({"is_array_buffer_instance":[true,true,true], "uint8array_array_buffer_and_input_arr_are_equal": [true,true,true]})
                .to_string(),
            ctx.get_handler_value()?
        );

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-slice-overflow.any.js
    #[test]
    fn test_response_slice_overflow() -> Result<()> {
        let mut ctx = Context::new();

        // Slice start is negative, relativeStart will be max((size + start), 0)
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-slice-overflow.any.js#L10
        ctx.eval(
            r#"
            var text = '';
            for (var i = 0; i < 2000; ++i) {
                text += 'A';
            }
            var blob = new Blob([text]);
            var sliceBlob = blob.slice(-1, blob.size);
            var sliceBlobSize = sliceBlob.size;
            "#,
        )?;

        assert_eq!("1", ctx.global.get_property("sliceBlobSize")?.as_str()?);

        // Slice start is greater than blob size, relativeStart will be min(start, size)
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-slice-overflow.any.js#L16
        ctx.eval(
            r#"
            var blob = new Blob([text]);
            var sliceBlob = blob.slice(blob.size + 1, blob.size);
            var sliceBlobSize = sliceBlob.size;
            "#,
        )?;

        assert_eq!("0", ctx.global.get_property("sliceBlobSize")?.as_str()?);

        // Slice start is greater than blob size, relativeStart will be min(start, size)
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-slice-overflow.any.js#L22
        ctx.eval(
            r#"
            var blob = new Blob([text]);
            var sliceBlob = blob.slice(blob.size - 2, -1);
            var sliceBlobSize = sliceBlob.size;
            "#,
        )?;

        assert_eq!("1", ctx.global.get_property("sliceBlobSize")?.as_str()?);

        // Slice start is greater than blob size, relativeStart will be min(start, size)
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-slice-overflow.any.js#L28
        ctx.eval(
            r#"
            var blob = new Blob([text]);
            var sliceBlob = blob.slice(blob.size - 2, blob.size + 999);
            var sliceBlobSize = sliceBlob.size;
            "#,
        )?;

        assert_eq!("2", ctx.global.get_property("sliceBlobSize")?.as_str()?);

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-slice.any.js
    #[test]
    fn test_response_slice() -> Result<()> {
        let mut ctx = Context::new();

        // With no-argument
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-slice.any.js#L5
        ctx.eval(
            r#"
            async function handler() {
                const blob = new Blob(["PASS"]);
                const sliceBlob = blob.slice();
                const sliceBlobText = await sliceBlob.text();

                return {sliceBlob: { size: sliceBlob.size, type: sliceBlob.type }, sliceBlobText};
            }
            "#,
        )?;

        assert_eq!(
            json!({"sliceBlob":{"size":4,"type":""}, "sliceBlobText":"PASS"}).to_string(),
            ctx.get_handler_value()?
        );

        // With type
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-slice.any.js#L25
        ctx.eval(
            r#"
            async function handler() {
                const blob = new Blob(["steak"], {type: "content/type"});
                const blobText = await blob.text();

                return {blob: { size: blob.size, type: blob.type }, blobText};
            }
            "#,
        )?;

        assert_eq!(
            json!({"blob":{"size":5,"type":"content/type"}, "blobText":"steak"}).to_string(),
            ctx.get_handler_value()?
        );

        // With null type Blob slice
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-slice.any.js#L33
        ctx.eval(
            r#"
            async function handler() {
                const blob = new Blob().slice(0,0,null);
                const blobText = await blob.text();

                return {blob: { size: blob.size, type: blob.type }, blobText};
            }
            "#,
        )?;

        assert_eq!(
            json!({"blob":{"size":0,"type":"null"}, "blobText":""}).to_string(),
            ctx.get_handler_value()?
        );

        // With undefined type Blob slice
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-slice.any.js#L41
        ctx.eval(
            r#"
            async function handler() {
                const blob = new Blob().slice(0,0,undefined);
                const blobText = await blob.text();

                return {blob: { size: blob.size, type: blob.type }, blobText};
            }
            "#,
        )?;

        assert_eq!(
            json!({"blob":{"size":0,"type":""}, "blobText":""}).to_string(),
            ctx.get_handler_value()?
        );

        // With no type Blob slice
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-slice.any.js#L49
        ctx.eval(
            r#"
            async function handler() {
                const blob = new Blob().slice(0,0);
                const blobText = await blob.text();

                return {blob: { size: blob.size, type: blob.type }, blobText};
            }
            "#,
        )?;

        assert_eq!(
            json!({"blob":{"size":0,"type":""}, "blobText":""}).to_string(),
            ctx.get_handler_value()?
        );

        // TODO: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-slice.any.js#L63
        // Should be covered by:
        // https://github.com/node-fetch/fetch-blob/blob/57e4daef36081936581d14509b6cc618d87ab9e2/test/test-wpt-in-node.js#L138

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-stream.any.js
    #[test]
    fn test_response_stream() -> Result<()> {
        let mut ctx = Context::new();

        // Utility function to read from a stream
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-stream.any.js#L18
        ctx.eval(
            r#"
            async function read_all_chunks(stream, perform_gc = false) {;
                const reader = stream.getReader();

                let read_value = await reader.read();

                let out = [];
                let i = 0;
                while (!read_value.done) {
                    for (let val of read_value.value) {
                        out[i++] = val;
                    }
                        read_value = await reader.read();
                }
                return out;
            }
            "#,
        )?;

        // Blob stream
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-stream.any.js#L37
        ctx.eval(
            r#"
            async function handler() {
                const blob = new Blob(["PASS"]);
                const blobStream = blob.stream();
                const chunks = await read_all_chunks(blobStream);
                
                let assert_equals = [];

                for (let [index, value] of chunks.entries()) {
                    assert_equals.push(value === "PASS".charCodeAt(index));
                }

                return {assert_equals};
            }
            "#,
        )?;

        assert_eq!(
            json!({"assert_equals":[true,true,true,true]}).to_string(),
            ctx.get_handler_value()?
        );

        // Blob stream empty Blob
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-stream.any.js#L46
        ctx.eval(
            r#"
            async function handler() {
                const blob = new Blob();
                const blobStream = blob.stream();
                const chunks = await read_all_chunks(blobStream);

                return {chunks};
            }
            "#,
        )?;

        assert_eq!(json!({"chunks":[]}).to_string(), ctx.get_handler_value()?);

        // Blob stream non-unicode input
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-stream.any.js#L53
        ctx.eval(
            r#"
            async function handler() {
                const input_arr = [8, 241, 48, 123, 151];
                const typed_arr = new Uint8Array(input_arr);
                const blob = new Blob([typed_arr]);
                const stream = blob.stream();
                const chunks = await read_all_chunks(stream);

                return {chunks};
            }
            "#,
        )?;

        assert_eq!(
            json!({"chunks":[8, 241, 48, 123, 151]}).to_string(),
            ctx.get_handler_value()?
        );
        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-text.any.js
    #[test]
    fn test_response_text() -> Result<()> {
        let mut ctx = Context::new();

        // Blob text
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-text.any.js#L5
        ctx.eval(
            r#"
            async function handler() {
                const blob = new Blob(["PASS"]);
                const blob_text = await blob.text();

                return {blob_text};
            }
            "#,
        )?;

        assert_eq!(
            json!({"blob_text":"PASS"}).to_string(),
            ctx.get_handler_value()?
        );

        // Blob text empty blob data
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-text.any.js#L11
        ctx.eval(
            r#"
            async function handler() {
                const blob = new Blob();
                const blob_text = await blob.text();

                return {blob_text};
            }
            "#,
        )?;

        assert_eq!(
            json!({"blob_text":""}).to_string(),
            ctx.get_handler_value()?
        );

        // Blob text multi-element array in constructor
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-text.any.js#L17
        ctx.eval(
            r#"
            async function handler() {
                const non_unicode = "\u0061\u030A";
                const input_arr = new TextEncoder().encode(non_unicode);
                const blob = new Blob([input_arr]);
                const blob_text = await blob.text();

                return {blob_text, non_unicode};
            }
            "#,
        )?;

        assert_eq!(
            json!({"blob_text":"a\u{30a}","non_unicode":"a\u{30a}"}).to_string(),
            ctx.get_handler_value()?
        );

        // Blob text non-unicode
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-text.any.js#L31
        ctx.eval(
            r#"
            async function handler() {
                const blob = new Blob(["PASS"], { type: "text/plain;charset=utf-16le" });
                const blob_text = await blob.text();

                return {blob_text};
            }
            "#,
        )?;

        assert_eq!(
            json!({"blob_text": "PASS"}).to_string(),
            ctx.get_handler_value()?
        );

        // Blob text different charset param with non-ascii input
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-text.any.js#L37
        ctx.eval(
            r#"
            async function handler() {
                const non_unicode = "\u0061\u030A";
                const input_arr = new TextEncoder().encode(non_unicode);
                const blob = new Blob([input_arr], { type: "text/plain;charset=utf-16le" });
                const blob_text = await blob.text();

                return {blob_text, non_unicode};
            }
            "#,
        )?;

        assert_eq!(
            json!({"blob_text":"a\u{30a}","non_unicode":"a\u{30a}"}).to_string(),
            ctx.get_handler_value()?
        );

        // Blob text invalid utf-8 input
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-text.any.js#L45
        ctx.eval(
            r#"
            async function handler() {
                const input_arr = new Uint8Array([192, 193, 245, 246, 247, 248, 249, 250, 251,252, 253, 254, 255]);
                const blob = new Blob([input_arr]);
                const blob_text = await blob.text();

                return {assert_equals: blob_text === "\ufffd\ufffd\ufffd\ufffd\ufffd\ufffd\ufffd\ufffd\ufffd" +
                "\ufffd\ufffd\ufffd\ufffd"};
            }
            "#,
        )?;

        assert_eq!(
            json!({"assert_equals": true}).to_string(),
            ctx.get_handler_value()?
        );

        // Blob text concurrent reads
        // @see: https://github.com/web-platform-tests/wpt/blob/3ccab5e25f082e8d055aef70b692a98786b9a9f2/FileAPI/blob/Blob-text.any.js#L54
        ctx.eval(
            r#"
            async function handler() {
                const input_arr = new Uint8Array([192, 193, 245, 246, 247, 248, 249, 250, 251,
                    252, 253, 254, 255]);
                const blob = new Blob([input_arr]);
                const text_results = await Promise.all([blob.text(), blob.text(),
                    blob.text()]);

                let assert_equals = [];

                for (let text of text_results) {
                  assert_equals.push(text === "\ufffd\ufffd\ufffd\ufffd\ufffd\ufffd\ufffd\ufffd\ufffd" +
                      "\ufffd\ufffd\ufffd\ufffd");
                }

                return {assert_equals};
            }
            "#,
        )?;

        assert_eq!(
            json!({"assert_equals": [true,true,true]}).to_string(),
            ctx.get_handler_value()?
        );

        Ok(())
    }
}
