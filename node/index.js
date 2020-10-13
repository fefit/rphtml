const rphtml = require("rphtml");
const htmlCode = `
<h1>hello.</h1>
<div class="header">
  <!--header-->
  <h3>this is header.</h3 >
</div>
`;
const ast = rphtml.parse(htmlCode, {
  allow_self_closing: true,
  allow_fix_unclose: false,
  case_sensitive_tagname: false,
});
const tree = ast.toJson();
console.log(tree);
const h1uid = tree.childs[1].uuid;
const h1 = ast.getTagByUuid(h1uid);
console.log("render h1===>", h1.render());
console.dir(JSON.stringify(tree, null, 4));
const doneCode = ast.render({
  always_close_void: false,
  lowercase_tagname: true,
  minify_spaces: true,
  remove_attr_quote: false,
  remove_comment: false,
  remove_endtag_space: true,
});
console.log(doneCode);