# rphtml

A html parser write in rust.

# use in node

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
const nodeList = rphtml.parse(htmlCode, {
  allow_self_closing: true,
  allow_fix_unclose: false,
  case_sensitive_tagname: false,
});

/*
// will output like this
{
  tag_index: 0,
  depth: 0,
  node_type: 'AbstractRoot',
  begin_at: { line_no: 1, col_no: 0 },
  end_at: { line_no: 1, col_no: 0 },
  end_tag: null,
  parent: null,
  content: null,
  childs: [
    {
      tag_index: 0,
      depth: 1,
      node_type: 'SpacesBetweenTag',
      begin_at: [Object],
      end_at: [Object],
      end_tag: null,
      parent: null,
      content: [Array],
      childs: null,
      meta: null,
      special: null
    },
    {
      tag_index: 1,
      depth: 2,
      node_type: 'Tag',
      begin_at: [Object],
      end_at: [Object],
      end_tag: [Object],
      parent: null,
      content: null,
      childs: [Array],
      meta: [Object],
      special: null
    },
    {
      tag_index: 0,
      depth: 1,
      node_type: 'SpacesBetweenTag',
      begin_at: [Object],
      end_at: [Object],
      end_tag: null,
      parent: null,
      content: [Array],
      childs: null,
      meta: null,
      special: null
    }
  ],
  meta: null,
  special: null
}
*/

const doneCode = rphtml.render(nodeList, {
  always_close_void: false,
  lowercase_tagname: true,
  minify_spaces: true,
  remove_attr_quote: false,
  remove_comment: false,
  remove_endtag_space: true,
});

/*
// output
<div class="header"><!--header--><h3>this is header.</h3></div>
*/
```

## API

### `parse(content: string, parseOptions: IParseOptions):IJsNode`

parse html code to nodes.

#### `IParseOptions`

- `allow_self_closing` if allow not void element use self-closing, e.g: `<div />`

- `allow_fix_unclose` if allow empty tags such as `<div><button class="btn"></div>`,not recommend

- `case_sensitive_tagname` if true, the tag's name will case-sensitive,that means `<div>` and `</DIV>` are not matched each other.

### `render(node: IJsNode, renderOptions: IRenderOptions):string`

render the node to html code.

#### `IRenderOptions`

- `always_close_void` always use self-closing for void elements.`<meta charset="utf8">` will output `<meta charset="utf8" />`

- `lowercase_tagname` if true, will always translate the tag's name to lowercase

- `minify_spaces` if true, will minify the spaces into one space if not in `pre` tag.

- `remove_attr_quote` if true, will remove the attribute value's quote `'` or `"`, if the value has special character such as spaces and `<` e.g, it will make no sense.

- `remove_comment` if true, will remove all comments node.

- `remove_endtag_space` if true, will remove the tag's end spaces, `<div></div >` will output `<div></div>`

#### `IJsNode`

```javascript
{
  tag_index: number; // the tag's index, just for element tag node.
  depth: number; // the node's depth of the nested.
  node_type: NodeType;
  begin_at: CodePosAt;
  end_at: CodePosAt;
  end_tag?: IJsNode; // the closed tag
  meta?: IJsTagMeta; // tag meta information.
  childs: Array<IJsNode>; // the childs of the tag.
}

```
