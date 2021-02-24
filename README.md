# rphtml

一个用 rust 编写的 html 文档解析器，0.4.0 版本前通过 wasm-pack/wasm-bindgen 提供 npm 包。

[![npm version](https://badge.fury.io/js/rphtml.svg)](https://badge.fury.io/js/rphtml)
[![Build Status](https://travis-ci.org/fefit/rphtml.svg?branch=master)](https://travis-ci.com/github/fefit/rphtml)
[![codecov](https://codecov.io/gh/fefit/rphtml/branch/master/graph/badge.svg)](https://codecov.io/gh/fefit/rphtml)

## 如何使用

```rust
use rphtml::parser::{ Doc, ParseOptions, RenderOptions, HResult };
fn main()->HResult{
  let doc = Doc::parse("<div id='content'>rpthml</div>", ParseOptions{
    case_sensitive_tagname: false, // 解析时标签区分大小写，`<div>` 和 `<DIV>` 将被视作不同标签，不建议开启
    allow_self_closing: false, // 允许非替换元素使用自闭合的写法，如 `<div class='' />`
    auto_fix_unclosed_tag: true, // 自动修复没有结束的标签，注意这里只是简单的将标签闭合
    auto_fix_unexpected_endtag: true, // 自动修复不正确的结束标签，如 "<div></p></div>" 会被修复为 "<div></div>"
    auto_fix_unescaped_lt: true, // 自动修复没有实体转译的左尖括号 '<', 比如`<div>a<b</div>`会被修复为`<div>a&lt;b</div>`
  })?;
  // 获取root根节点
  let root = doc.get_root_node();
  let render_html = doc.render(&RenderOptions{
    ..Default::default() // RenderOptions的参数定义可以在wiki中查看
  });
}
```

## License

[MIT License](./LICENSE).
