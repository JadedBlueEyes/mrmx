use expect_test::expect;
use mrmx::{view, WithAttribute};

#[test]
fn basic_html() {
    let expected = expect![[r#"Node { tag: "p", attributes: Map({}), children: [] }"#]];
    let actual: mrml::node::Node<mrml::mj_body::MjBodyChild> = view! {<p></p>};

    expected.assert_eq(&format!("{actual:?}"))
}

#[test]
fn mjml() {
    let expected = expect!["Mjml { attributes: MjmlAttributes { owa: None, lang: None, dir: None }, children: MjmlChildren { head: None, body: None } }"];
    let actual = view! { <mjml> </mjml> };

    expected.assert_eq(&format!("{actual:?}"))
}

#[test]
fn mjml_title() {
    let expected = expect![[
        r#"Mjml { attributes: MjmlAttributes { owa: None, lang: None, dir: None }, children: MjmlChildren { head: Some(MjHead { children: [MjTitle(MjTitle { children: "title" })] }), body: None } }"#
    ]];
    let actual = view! {
      <mjml>
        <mj-head>
          <mj-title>title</mj-title>
        </mj-head>
      </mjml>
    };

    expected.assert_eq(&format!("{actual:?}"))
}

#[test]
fn mjml_conditional() {
    let expected = expect![[
        r#"Mjml { attributes: MjmlAttributes { owa: None, lang: None, dir: None }, children: MjmlChildren { head: Some(MjHead { children: [MjTitle(MjTitle { children: "title" })] }), body: Some(MjBody { attributes: Map({}), children: [MjButton(MjButton { attributes: Map({}), children: [Text(Text("Hi"))] })] }) } }"#
    ]];
    let actual = view! {
      <mjml>
        <mj-head>
          <mj-title>title</mj-title>
        </mj-head>
        <mj-body> {
          if true {
            view!{<mj-button>"Hi"</mj-button>}.into()
          } else {
            view!{ "Bye" }.into()
          }
        } </mj-body>
      </mjml>
    };

    expected.assert_eq(&format!("{actual:?}"))
}

#[test]
fn mjml_doc() {
    let expected = expect![[
        r#"Mjml { attributes: MjmlAttributes { owa: None, lang: None, dir: None }, children: MjmlChildren { head: Some(MjHead { children: [MjTitle(MjTitle { children: "It's a title!" }), MjAttributes(MjAttributes { children: [MjAttributesElement(MjAttributesElement { name: "mj-text", attributes: Map({"padding": "0"}) }), MjAttributesAll(MjAttributesAll { attributes: Map({"font-family": "serif"}) }), MjAttributesClass(MjAttributesClass { name: "heading", attributes: Map({"color": "red"}) })] })] }), body: Some(MjBody { attributes: Map({}), children: [MjSection(MjSection { attributes: Map({}), children: [MjColumn(MjColumn { attributes: Map({}), children: [MjText(MjText { attributes: Map({"mj-class": "heading"}), children: [Text(Text("coucou"))] })] })] })] }) } }"#
    ]];
    let actual = view! {
      <mjml>
        <mj-head>
          <mj-title>"It's a title!"</mj-title>
          <mj-attributes>
            <mj-text padding="0" />
            <mj-all font-family="serif" />
            <mj-class name="heading" color="red" />
          </mj-attributes>
        </mj-head>
        <mj-body>
          <mj-section>
            <mj-column>
              <mj-text mj-class="heading">
                coucou
              </mj-text>
            </mj-column>
          </mj-section>
        </mj-body>
      </mjml>
    };

    expected.assert_eq(&format!("{actual:?}"))
}
