# rphtml

A html parser written in rust, build wasm code for npm package.

一个用 rust 编写的 html 解析器，通过 wasm-pack/wasm-bindgen 提供 npm 包。

[![npm version](https://badge.fury.io/js/rphtml.svg)](https://badge.fury.io/js/rphtml)
[![Build Status](https://travis-ci.org/fefit/rphtml.svg?branch=master)](https://travis-ci.com/github/fefit/rphtml)
[![codecov](https://codecov.io/gh/fefit/rphtml/branch/master/graph/badge.svg)](https://codecov.io/gh/fefit/rphtml)

## Use in node

```bash
# npm
npm install rphtml --save

# yarn
yarn add rphtml
```

```javascript
import rphtml from "rphtml";
const htmlCode = `
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

const jsonData = ast.toJson();
console.log(jsonData);
/*
// will output like this
{ tag_index: 0,
  depth: 0,
  node_type: 'AbstractRoot',
  begin_at: { line_no: 1, col_no: 0, index: 0 },
  end_at: { line_no: 6, col_no: 0, index: 72 },
  childs:
   [ { tag_index: 0,
       depth: 1,
       node_type: 'SpacesBetweenTag',
       begin_at: [Object],
       end_at: [Object],
       content: [Array] },
     { tag_index: 1,
       depth: 2,
       node_type: 'Tag',
       begin_at: [Object],
       end_at: [Object],
       end_tag: [Object],
       childs: [Array],
       meta: [Object] },
     { tag_index: 0,
       depth: 1,
       node_type: 'SpacesBetweenTag',
       begin_at: [Object],
       end_at: [Object],
       content: [Array] } ] }
*/

const code = ast.render(nodeList, {
  always_close_void: false,
  lowercase_tagname: true,
  minify_spaces: true,
  remove_attr_quote: false,
  remove_comment: false,
  remove_endtag_space: true,
});
console.log(code);

/*
// output
<div class="header"><!--header--><h3>this is header.</h3></div>
*/
```

## API

### Methods

### `parse(content: string, parseOptions?: IJsParseOptions) : IJsNode`

parse html code to AST, it's a pointer.

通过 `parse` 静态方法将 html 字符串解析成 html 解析树，它的返回值是一个指针对象，需要调用其上的方法才能获得实际可用的数据。

---

### `IJsParseOptions`

```typescript
// static 'parse' method argument options.
// parse方法提供以下配置参数
type IJsParserOptions = {
  allow_self_closing?: boolean;
  allow_fix_unclose?: boolean;
  case_sensitive_tagname?: boolean;
  decode_entity?: boolean;
};
```

- `allow_self_closing`

  if allow not void element use self-closing, e.g.: `<div />`

  是否允许自闭合标签，不为 true 的情况下 `<div />` 这种写法将视为错误。

- `allow_fix_unclose`

  if allow empty tags such as `<div><button class="btn"></div>`,not recommend

  是否允许自动修复没有闭合的标签，不推荐，因为这种修复很可能是错误的。

- `case_sensitive_tagname`

  if true, the tag's name will case-sensitive,that means `<div>` and `</DIV>` are not matched each other.

  tag 标签是否区分大小写，区分大小写的情况下，`<div>`和`<DIV>` 将视作不同的标签，将影响标签的配对。

- `decode_entity`

  if true, will decode the entity text to an unicode character.

  是否 decode 文本中的实体为一个 unicode 字符。

---

### `IJsNode`

### Methods of IJsNode

#### `render(renderOptions?: IJsRenderOptions) : string`

render the ast to html code.

将 html 解析树渲染成 html 代码。

---

#### `IJsRenderOptions`

```typescript
type IJsRenderOptions = {
  always_close_void?: boolean;
  lowercase_tagname?: boolean;
  minify_spaces?: boolean;
  remove_attr_quote?: boolean;
  remove_comment?: boolean;
  remove_endtag_space?: boolean;
  inner_html?: boolean;
};
```

- `always_close_void`

  always use self-closing for void elements.`<meta charset="utf8">` will output `<meta charset="utf8" />`

  为 true 时，将始终给 void elements 标签元素添加反斜杠自闭合标记。

- `lowercase_tagname`

  if true, will always translate the tag's name to lowercase

  为 true 时，将强制将所有标签名转为小写。

- `minify_spaces`

  if true, will remove all the spaces between tags and minify the text node's repeat spaces into one if not in `pre` tag.

  为 true 时，将去除标签之间的空格；标签内文本左右的空格将会压缩成一个；pre 标签的空格将保持不变。

- `remove_attr_quote`

  if true, will remove the attribute value's quote `'` or `"`, if the value has special character such as spaces and `<` e.g, it will make no sense.

  为 true 时，将会根据属性值有条件的去掉引号。

- `remove_comment`

  if true, will remove all comments node.

  是否移除所有注释标签。

- `remove_endtag_space`

  if true, will remove the tag's end spaces, `<div></div >` will output `<div></div>`

  由于结束标签后面允许出现空格，设为 true 的情况下将会去除这些空格。

- `inner_html`

  if true,will output the tag's inner html.

  为 true 的情况下，将获取 innerHTML.

---

### `toJson() : IJsNodeTree`

return a json data from the pointer after call the `parse` method.

通过`parse`方法得到了指针对象后，调用`toJson()`方法可以获得一个 json 格式的解析树数据。

```typescript
type IJsNodeTree = {
  uuid?: string; // the tag's uuid, only for element tag node.
  depth: number; // the node's depth of the nested.
  node_type: NodeType;
  begin_at: CodePosAt;
  end_at: CodePosAt;
  content?: Array<string>; // content character
  end_tag?: IJsNodeTree; // the closed tag
  meta?: IJsNodeTagMeta; // tag meta information.
  childs?: Array<IJsNodeTree>; // the childs of the tag.
};
```

---

### `toString() : string`

return the string of the json data, like `JSON.stringify()`, you can use `JSON.parse()` in javascript to get the json data,is same as the `toJSON()`.

和`toJson`方法类似，但该方法返回的是 json 字符串，需要进行 parse 后才能获得真实的 json 对象。

---

### `getTagByUuid(uuid:string) : IJsNode`

return the tag node by uuid.

每个 tag 节点都会带有自己的 uuid 标记，通过调用根节点的该方法可以获得子节点的引用。与 dom 内的`getElementById`方法类似。

---

### `isAloneTag: boolean`

check if the node is a tag node without child tags, e.g. `<div>abc</div>`

属性`isAloneTag`表示标签 tag 不包含有其它子 tag。

---

## License

[MIT License](./LICENSE).
