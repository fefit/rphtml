const rxhtml = require("rxhtml");
const htmlStrNodes = rxhtml.parse(`
    <!DOCTYPE html>
    <html>
        <head>
            <title>测试页面</title>
        </head>
        <body>
            <!--注释-->
            <div class="header">
                测试header
            </DIV   >
        </body>
    </html>
`,{
    allow_self_closing: true
});
console.log(JSON.stringify(htmlStrNodes, null, 4));
const output = rxhtml.render(htmlStrNodes, {
    minify_spaces: true
});
console.log(output);