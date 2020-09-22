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
`);
console.log(JSON.stringify(htmlStrNodes, null, 4));
const output = rxhtml.render(htmlStrNodes, {
    minify_spaces: false,
    lowercase_tagname: false,
    remove_endtag_space: false,
    remove_attr_quote: false,
    remove_comment: false
});
console.log(output);