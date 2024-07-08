# MRMX

[![CI status](https://github.com/JadedBlueEyes/mrmx/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/JadedBlueEyes/mrmx/actions/workflows/ci.yml?query=branch%3Amain) [![dependency status](https://deps.rs/repo/github/JadedBlueEyes/mrmx/status.svg?path=crates%2Fmrmx)](https://deps.rs/repo/github/JadedBlueEyes/mrmx?path=crates%2Fmrmx) [![Last commit](https://img.shields.io/github/last-commit/JadedBlueEyes/mrmx.svg?logo=github&logoColor=white)](https://github.com/JadedBlueEyes/mrmx/commits/main/) [![pre-commit enabled](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit)](https://github.com/pre-commit/pre-commit)

<!-- cargo-rdme start -->

The mrmx crate provides a JSX-like syntax for generating Mjml.

It allows generating subsections of a document:

```rust
view! { <mj-title>title</mj-title> };
```

Complete documents:

```rust
view! {
    <mjml>
        <mj-body>
            <mj-text>Hello world!</mj-text>
        </mj-body>
    </mjml>
   };
```

And interpolating multiple trees:


```rust
let title = view! { <mj-title>title</mj-title> };
view! {
    <mjml>
        <mj-head>
            { title.into() }
        </mj-head>
        <mj-body>
            <!-- "Single quotes must be contained in strings" -->
            <mj-text>"Isn't this cool?"</mj-text>
        </mj-body>
    </mjml>
};
```

You can also embed blocks of arbitrary code inside trees:

```rust
view! {
    <mjml>
        <mj-head>
            <mj-title>title</mj-title>
        </mj-head>
        <mj-body> {
            if 4 < 5 {
                view!{ <mj-button>"Maths works!"</mj-button> }.into()
            } else {
                view!{ "Oh no!" }.into()
            }
        } </mj-body>
    </mjml>
};
```

<!-- cargo-rdme end -->

License: MIT OR Apache-2.0
