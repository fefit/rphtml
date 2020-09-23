const rxhtml = require("rxhtml");
const htmlStrNodes = rxhtml.parse(`
    <!DOCTYPE html>
    <html>
        <head>
            <title>测试页面</title>
        </head>
        <body>
            <div class="header">
                测试header
            </div>
        </body>
    </html>
`, {
    case_sensitive_tagname: false,
    allow_self_closing: false,
    allow_fix_unclose: false
});
console.log(JSON.stringify(htmlStrNodes, null, 4));
const output = rxhtml.render(htmlStrNodes, {
    minify_spaces: false,
    lowercase_tagname: false,
    remove_endtag_space: false,
    remove_attr_quote: false,
    remove_comment: false,
    always_close_void: false
});
console.log(output);