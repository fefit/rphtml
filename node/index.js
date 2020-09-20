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
console.log(htmlStrNodes);