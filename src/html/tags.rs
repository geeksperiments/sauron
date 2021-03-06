//! provides functionalities and macro for building html elements
macro_rules! declare_tags {
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        $(
            doc_comment!{
                concat!("Creates an html [",stringify!($name),"](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/",stringify!($name),") element"),

            $(#[$attr])*
            #[inline]
            #[allow(non_snake_case)]
            pub fn $name<MSG>(attrs: Vec<$crate::Attribute<MSG>>, children: Vec<$crate::Node<MSG>>) -> $crate::Node<MSG>
                {
                    $crate::html::html_element(stringify!($name), attrs, children)
                }
            }

         )*
    }
}

/// declare an element tag in a macro
/// This will create a macro which creates an element with the tag name
/// which matches the function name.
/// Example:
/// ```rust,ignore
/// div!([], [])
/// ```
/// is a macro which creates a `div` element tag
/// This macro is generated using
/// ```rust,ignore
/// declare_tags_macro!{div;}
/// ```
///
/// Note: The $ dollar sign is explcitly pass to prevent
/// rustc to attempt to expand the inner repetion of the macro
macro_rules! declare_tags_macro {
    (($d:tt) $($name: ident;)*) => {
        $(

            #[macro_export]
            /// TODO:
            macro_rules! $name {

                // 000: no trailing commas
                ( [$d($att: expr),*], [$d($children: expr),*] ) => {
                    $crate::html::$name(vec![$d($att),*], vec![$d($children),*])
                };

                ///////////////////////////////////////////////////////////////
                //
                // The next code is just the same logic as the first, it is just
                // here to deal with irregular comma placement
                //
                ///////////////////////////////////////////////////////////////

                // 001: trailing commas in params only
                ( [$d($att: expr),*], [$d($children: expr),*], ) => {
                    $crate::html::$name(vec![$d($att),*], vec![$d($children),*])
                };
                // 010: trailing commas in children only
                ( [$d($att: expr),*], [$d($children: expr,)*] ) => {
                    $crate::html::$name(vec![$d($att),*], vec![$d($children),*])
                };
                // 011: trailing commas in children and params,
                ( [$d($att: expr),*], [$d($children: expr,)*], ) => {
                    $crate::html::$name(vec![$d($att),*], vec![$d($children),*])
                };
                // 100: trailing commas in attributes only
                ( [$d($att: expr,)*], [$d($children: expr),*] ) => {
                    $crate::html::$name(vec![$d($att),*], vec![$d($children),*])
                };
                // 101: trailing commas in attributes and params,
                ( [$d($att: expr,)*], [$d($children: expr,)*], ) => {
                    $crate::html::$name(vec![$d($att),*], vec![$d($children),*])
                };
                // 110: trailing commas in attributes and children
                ( [$d($att: expr,)*], [$d($children: expr,)*] ) => {
                    $crate::html::$name(vec![$d($att),*], vec![$d($children),*])
                };
                // 111: trailing commas in attributes, children, params
                ( [$d($att: expr,)*], [$d($children: expr,)*], ) => {
                    $crate::html::$name(vec![$d($att),*], vec![$d($children),*])
                };

                /////////////////////////////////////////////////
                //
                // Pass through the expression as it was with the old function call
                //
                /////////////////////////////////////////////////

                // Pass through the div(vec![], vec![])
                ( $att: expr, $children: expr ) => {
                    $crate::html::$name( $att, $children)
                };

                // Pass through the div!(vec![], vec![],) with trailing comma
                ( $att: expr, $children: expr, ) => {
                    $crate::html::$name( $att, $children)
                };
            }
        )*
    };
}

macro_rules! declare_common_tags_and_macro {
    ($($(#[$attr:meta])* $name:ident;)*) => {

        pub(crate) mod commons {
            declare_tags! { $($name;)* }

            pub(crate) mod macros{
                // we passed `$` which is the $d:tt in declare_tags_macro
                // passing the `$` token is needed so as to prevent the called macro from unrolling
                // it.
                declare_tags_macro! {($) $($name;)* }
            }
        }

        #[cfg(feature = "with-parser")]
        /// These are the comonly used html tags such as div, input, buttons,.. etc
        pub const HTML_TAGS: [&'static str; 112] = [$(stringify!($name),)*];
    };
}

macro_rules! declare_tags_and_macro {
    ($($(#[$attr:meta])* $name:ident;)*) => {

        declare_tags! { $($name;)* }

        declare_tags_macro! {($) $($name;)* }

    };
}

macro_rules! declare_tags_non_common{

    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        declare_tags!{ $($name;)*}

        #[cfg(feature = "with-parser")]
        /// These are html tags which are non commonly used.
        /// Put together in this collection to avoid import conflicts with the commonly used
        /// ones.
        pub const HTML_TAGS_NON_COMMON:[&'static str;1] = [$(stringify!($name),)*];
    }
}

macro_rules! declare_tags_and_macro_non_common{

    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        declare_tags_and_macro!{ $($name;)*}

        #[cfg(feature = "with-parser")]
        /// These are html tags with macro which are non commonly used.
        /// Put together in this collection to avoid import conflicts with the commonly used
        /// ones.
        pub const HTML_TAGS_WITH_MACRO_NON_COMMON:[&'static str;2] = [$(stringify!($name),)*];
    }
}

// Organized in the same order as
// https://developer.mozilla.org/en-US/docs/Web/HTML/Element
//
// Does not include obsolete elements.
declare_common_tags_and_macro! {
    base;
    head;
    link;
    meta;
    body;
    address;
    article;
    aside;
    footer;
    header;
    h1;
    h2;
    h3;
    h4;
    h5;
    h6;
    hgroup;
    main;
    nav;
    section;
    blockquote;
    dd;
    div;
    dl;
    dt;
    figcaption;
    figure;
    hr;
    html;
    li;
    ol;
    p;
    pre;
    ul;
    a;
    abbr;
    b;
    bdi;
    bdo;
    br;
    cite;
    code;
    data;
    dfn;
    em;
    i;
    kbd;
    mark;
    q;
    rb;
    rp;
    rt;
    rtc;
    ruby;
    s;
    samp;
    small;
    span;
    strong;
    sub;
    sup;
    time;
    u;
    var;
    wbr;
    area;
    audio;
    img;
    map;
    track;
    video;
    embed;
    iframe;
    object;
    param;
    picture;
    source;
    canvas;
    noscript;
    script;
    del;
    ins;
    caption;
    col;
    colgroup;
    table;
    tbody;
    td;
    tfoot;
    th;
    thead;
    tr;
    button;
    datalist;
    fieldset;
    form;
    input;
    label;
    legend;
    meter;
    optgroup;
    option;
    output;
    progress;
    select;
    textarea;
    details;
    dialog;
    menu;
    menuitem;
    summary;
    template;
}

declare_tags_non_common! {
    style;  //  conflicts with html::attributes::style, attribute::style    > tags::style
}

// These are non-common tags
// which the users need to explicitly import using
// html::tags::style, html::tags::html, etc
//
declare_tags_and_macro_non_common! {
    title; // conflicts with html::attributes::title  , attributes::title   > tags::title
    slot;  // conflicts with html::attributes::slot   , attrributes::slot   > tags::slot
}
