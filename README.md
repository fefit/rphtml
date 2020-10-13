# rphtml

A html parser write in rust, build wasm code for npm package.

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
  begin_at: { line_no: 1, col_no: 0, index: -1 },
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

- ### `parse(content: string, parseOptions?: IJsParseOptions) : IJsNode`

  parse html code to nodes.

  - #### argument: `IJsParseOptions`

    - `allow_self_closing` if allow not void element use self-closing, e.g: `<div />`

    - `allow_fix_unclose` if allow empty tags such as `<div><button class="btn"></div>`,not recommend

    - `case_sensitive_tagname` if true, the tag's name will case-sensitive,that means `<div>` and `</DIV>` are not matched each other.

### return value: `IJsNode`

- ### `render(renderOptions?: IJsRenderOptions) : string`

  render the node to html code.

  - #### render function argument: `IJsRenderOptions`

    - `always_close_void` always use self-closing for void elements.`<meta charset="utf8">` will output `<meta charset="utf8" />`

    - `lowercase_tagname` if true, will always translate the tag's name to lowercase

    - `minify_spaces` if true, will minify the spaces into one space if not in `pre` tag.

    - `remove_attr_quote` if true, will remove the attribute value's quote `'` or `"`, if the value has special character such as spaces and `<` e.g, it will make no sense.

    - `remove_comment` if true, will remove all comments node.

    - `remove_endtag_space` if true, will remove the tag's end spaces, `<div></div >` will output `<div></div>`

- ### `toJson() : IJsNodeTree`

  ```javascript
  {
    uuid?: string; // the tag's uuid, only for element tag node.
    depth: number; // the node's depth of the nested.
    node_type: NodeType;
    begin_at: CodePosAt;
    end_at: CodePosAt;
    end_tag?: IJsNodeTree; // the closed tag
    meta?: IJsNodeTagMeta; // tag meta information.
    childs?: Array<IJsNodeTree>; // the childs of the tag.
  }

  ```

- ### `toString() : string`

  return the string of the json data, like `JSON.stringify()`, you can use `JSON.parse()` in javascript to get the json data,is same as the `toJSON()`.

- ### `getTagByUuid(uuid:string) : IJsNode`
  return the tag node by uuid.

## License

[MIT License](./LICENSE).
