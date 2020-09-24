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
`);
console.log(JSON.stringify(htmlStrNodes, null, 4));
const output = rxhtml.render(htmlStrNodes, rxhtml.JsRenderOptions.default());
console.log(output);