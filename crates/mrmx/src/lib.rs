//! # mrmx
//!
//! The mrmx crate provides a JSX-like syntax for generating Mjml.
//!
//! It allows generating subsections of a document:
//!
//! ```
//! # use mrmx_macros::view;
//! view! { <mj-title>title</mj-title> };
//! ```
//!
//! Complete documents:
//!
//! ```
//! # use mrmx_macros::view;
//! view! {
//!     <mjml>
//!         <mj-body>
//!             <mj-text>Hello world!</mj-text>
//!         </mj-body>
//!     </mjml>
//!    };
//! ```
//!
//! And interpolating multiple trees:
//!
//!
//! ```
//! # use mrmx_macros::view;
//! #
//! let title = view! { <mj-title>title</mj-title> };
//! view! {
//!     <mjml>
//!         <mj-head>
//!             { title.into() }
//!         </mj-head>
//!         <mj-body>
//!             <!-- "Single quotes must be contained in strings" -->
//!             <mj-text>"Isn't this cool?"</mj-text>
//!         </mj-body>
//!     </mjml>
//! };
//! ```
//!
//! You can also embed blocks of arbitrary code inside trees:
//!
//! ```
//! # use mrmx_macros::view;
//! view! {
//!     <mjml>
//!         <mj-head>
//!             <mj-title>title</mj-title>
//!         </mj-head>
//!         <mj-body> {
//!             if 4 < 5 {
//!                 view!{ <mj-button>"Maths works!"</mj-button> }.into()
//!             } else {
//!                 view!{ "Oh no!" }.into()
//!             }
//!         } </mj-body>
//!     </mjml>
//! };
//! ```
//!

#[cfg(feature = "macros")]
pub use mrmx_macros::view;

pub trait WithChildren {
    type Child;

    fn with_children(self, children: Vec<Self::Child>) -> Self;
}

impl<T> WithChildren for mrml::node::Node<T> {
    type Child = T;

    fn with_children(mut self, mut children: Vec<T>) -> Self {
        self.children.append(&mut children);
        self
    }
}

pub enum MjmlChild {
    Head(mrml::mj_head::MjHead),
    Body(mrml::mj_body::MjBody),
}

impl From<mrml::mj_head::MjHead> for MjmlChild {
    fn from(value: mrml::mj_head::MjHead) -> Self {
        MjmlChild::Head(value)
    }
}
impl From<mrml::mj_body::MjBody> for MjmlChild {
    fn from(value: mrml::mj_body::MjBody) -> Self {
        MjmlChild::Body(value)
    }
}

impl WithChildren for mrml::mjml::Mjml {
    type Child = MjmlChild;

    fn with_children(mut self, children: Vec<MjmlChild>) -> Self {
        for ch in children {
            match ch {
                MjmlChild::Head(head) => self.children.head = Some(head),
                MjmlChild::Body(body) => self.children.body = Some(body),
            }
        }
        self
    }
}

#[derive(Debug)]
// , serde::Serialize, serde::Deserialize)]
// #[serde(untagged)]
pub enum MjAccordionElementChild {
    Comment(mrml::comment::Comment),
    MjAccordionText(mrml::mj_accordion_text::MjAccordionText),
    MjAccordionTitle(mrml::mj_accordion_title::MjAccordionTitle),
}

impl WithChildren for mrml::mj_accordion_element::MjAccordionElement {
    type Child = MjAccordionElementChild;

    fn with_children(mut self, children: Vec<MjAccordionElementChild>) -> Self {
        for ch in children {
            match ch {
                MjAccordionElementChild::Comment(_) => {}
                MjAccordionElementChild::MjAccordionText(text) => self.children.text = Some(text),
                MjAccordionElementChild::MjAccordionTitle(title) => {
                    self.children.title = Some(title)
                }
            }
        }
        self
    }
}

macro_rules! with_children {
    ($el:path, $ch:path ) => {
        impl WithChildren for $el {
            type Child = $ch;

            fn with_children(mut self, mut children: Vec<$ch>) -> Self {
                self.children.append(&mut children);
                self
            }
        }
    };
}

with_children!(mrml::mj_body::MjBody, mrml::mj_body::MjBodyChild);
with_children!(mrml::mj_head::MjHead, mrml::mj_head::MjHeadChild);

with_children!(mrml::mj_text::MjText, mrml::mj_body::MjBodyChild);
with_children!(mrml::mj_button::MjButton, mrml::mj_body::MjBodyChild);
with_children!(mrml::mj_section::MjSection, mrml::mj_body::MjBodyChild);
with_children!(mrml::mj_column::MjColumn, mrml::mj_body::MjBodyChild);
with_children!(mrml::mj_group::MjGroup, mrml::mj_body::MjBodyChild);
with_children!(mrml::mj_hero::MjHero, mrml::mj_body::MjBodyChild);
with_children!(mrml::mj_table::MjTable, mrml::mj_body::MjBodyChild);
with_children!(mrml::mj_wrapper::MjWrapper, mrml::mj_body::MjBodyChild);

with_children!(
    mrml::mj_accordion_text::MjAccordionText,
    mrml::mj_raw::MjRawChild
);
with_children!(mrml::mj_navbar_link::MjNavbarLink, mrml::mj_raw::MjRawChild);
with_children!(
    mrml::mj_social_element::MjSocialElement,
    mrml::mj_raw::MjRawChild
);
with_children!(mrml::mj_raw::MjRaw, mrml::mj_raw::MjRawChild);

with_children!(
    mrml::mj_attributes::MjAttributes,
    mrml::mj_attributes::MjAttributesChild
);

with_children!(mrml::mj_accordion_title::MjAccordionTitle, mrml::text::Text);
with_children!(
    mrml::mj_accordion::MjAccordion,
    mrml::mj_accordion::MjAccordionChild
);
with_children!(
    mrml::mj_carousel::MjCarousel,
    mrml::mj_carousel::MjCarouselChild
);
with_children!(mrml::mj_navbar::MjNavbar, mrml::mj_navbar::MjNavbarChild);
with_children!(mrml::mj_social::MjSocial, mrml::mj_social::MjSocialChild);
with_children!(
    mrml::mj_include::body::MjIncludeBody,
    mrml::mj_include::body::MjIncludeBodyChild
);
with_children!(
    mrml::mj_include::head::MjIncludeHead,
    mrml::mj_include::head::MjIncludeHeadChild
);

// mrml::mj_attributes_all::MjAttributesAll
// mrml::mj_attributes_class::MjAttributesClass
// mrml::mj_attributes_element::MjAttributesElement

// mrml::mj_carousel_image::MjCarouselImage
// mrml::mj_image::MjImage
// mrml::mj_divider::MjDivider
// mrml::mj_spacer::MjSpacer

// mrml::mj_breakpoint::MjBreakpoint
// mrml::mj_font::MjFont
// mrml::mj_style::MjStyle

// mrml::mj_title::MjTitle
// mrml::mj_preview::MjPreview

pub trait WithAttribute {
    fn with_attribute(self, key: String, value: String) -> Self;
}

impl<T> WithAttribute for mrml::node::Node<T> {
    fn with_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }
}

macro_rules! with_attribute {
    ($el:path ) => {
        impl WithAttribute for $el {
            fn with_attribute(mut self, key: String, value: String) -> Self {
                self.attributes.insert(key, value);
                self
            }
        }
    };
}

with_attribute!(mrml::mj_body::MjBody);

with_attribute!(mrml::mj_text::MjText);
with_attribute!(mrml::mj_button::MjButton);
with_attribute!(mrml::mj_section::MjSection);
with_attribute!(mrml::mj_column::MjColumn);
with_attribute!(mrml::mj_group::MjGroup);
with_attribute!(mrml::mj_hero::MjHero);
with_attribute!(mrml::mj_table::MjTable);
with_attribute!(mrml::mj_wrapper::MjWrapper);
with_attribute!(mrml::mj_accordion::MjAccordion);

with_attribute!(mrml::mj_accordion_text::MjAccordionText);
with_attribute!(mrml::mj_navbar_link::MjNavbarLink);
with_attribute!(mrml::mj_social_element::MjSocialElement);

with_attribute!(mrml::mj_accordion_title::MjAccordionTitle);
with_attribute!(mrml::mj_carousel::MjCarousel);
with_attribute!(mrml::mj_navbar::MjNavbar);
with_attribute!(mrml::mj_social::MjSocial);
with_attribute!(mrml::mj_attributes_all::MjAttributesAll);

impl WithAttribute for mrml::mj_attributes_class::MjAttributesClass {
    fn with_attribute(mut self, key: String, value: String) -> Self {
        if key == "name" {
            self.name = value;
            return self;
        }
        self.attributes.insert(key, value);
        self
    }
}

impl WithAttribute for mrml::mj_attributes_element::MjAttributesElement {
    fn with_attribute(mut self, key: String, value: String) -> Self {
        if key == "name" {
            self.name = value;
            return self;
        }
        self.attributes.insert(key, value);
        self
    }
}

with_attribute!(mrml::mj_carousel_image::MjCarouselImage);
with_attribute!(mrml::mj_image::MjImage);
with_attribute!(mrml::mj_divider::MjDivider);
with_attribute!(mrml::mj_spacer::MjSpacer);

impl WithAttribute for mrml::mj_breakpoint::MjBreakpoint {
    fn with_attribute(mut self, key: String, value: String) -> Self {
        if key == "width" {
            self.attributes.width = value;
            return self;
        }
        // self.attributes.insert(key, value);
        self
    }
}

impl WithAttribute for mrml::mj_style::MjStyle {
    fn with_attribute(mut self, key: String, value: String) -> Self {
        if key == "inline" {
            self.attributes.inline = Some(value);
            return self;
        }
        // self.attributes.insert(key, value);
        self
    }
}

impl WithAttribute for mrml::mj_font::MjFont {
    fn with_attribute(mut self, key: String, value: String) -> Self {
        if key == "name" {
            self.attributes.name = value;
            return self;
        }
        if key == "href" {
            self.attributes.href = value;
            return self;
        }
        // self.attributes.insert(key, value);
        self
    }
}

// mrml::mj_title::MjTitle
// mrml::mj_preview::MjPreview
